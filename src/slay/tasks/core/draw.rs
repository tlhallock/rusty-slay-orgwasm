use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::status_effects::effect::PlayerStatusEffect;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::choose::ChooseTask;
use crate::slay::tasks::tasks::immediate;

fn player_can_play_immediately(
	game: &mut Game,
	player_index: ids::PlayerIndex,
	card: &Card,
) -> bool {
	(game.player_has_effect(player_index, PlayerStatusEffect::PlayMagicOnDraw)
		&& matches!(card.card_type, SlayCardSpec::MagicCard(_)))
		|| (game.player_has_effect(player_index, PlayerStatusEffect::PlayItemOnDraw)
			&& matches!(card.card_type, SlayCardSpec::Item(_)))
}

fn player_has_drawn(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
	card: &Card,
) {
	if player_can_play_immediately(game, player_index, card) {
		if let Some(choices) =
			immediate::create_play_immediately_choices(context, game, player_index, card, None)
		{
			game.players[player_index]
				.tasks
				.prepend(ChooseTask::create(choices));
		}
	}
}

#[derive(Debug, Clone)]
enum DrawAmount {
	Fixed(usize),
	Until(usize),
}

#[derive(Debug, Clone)]
pub struct DrawTask {
	amount: DrawAmount,
	param: Option<TaskParamName>,
}

impl DrawTask {
	pub fn create(num: usize) -> Box<dyn PlayerTask> {
		Box::new(Self::new(num))
	}
	pub fn new(number_to_draw: usize) -> Self {
		Self {
			amount: DrawAmount::Fixed(number_to_draw),
			param: None,
		}
	}
	pub fn until(until_amount: usize) -> Box<dyn PlayerTask> {
		Box::new(Self {
			amount: DrawAmount::Until(until_amount),
			param: None,
		})
	}
	pub fn into_param(param: TaskParamName) -> Box<dyn PlayerTask> {
		Box::new(Self {
			amount: DrawAmount::Fixed(1),
			param: Some(param),
		})
	}
	fn cannot_draw_at_all(&mut self, hand_size: usize) -> bool {
		match self.amount {
			// TODO: Double check this...
			DrawAmount::Fixed(_) => false, // amount <= 0,
			DrawAmount::Until(amount) => amount >= hand_size,
		}
	}
	fn decrement_and_check_if_is_last_draw(&mut self, hand_size: usize) -> bool {
		let (new_amount, is_last) = match self.amount {
			DrawAmount::Fixed(amount) => (DrawAmount::Fixed(amount - 1), amount <= 1),
			DrawAmount::Until(amount) => (DrawAmount::Until(amount), hand_size >= amount - 1),
		};
		self.amount = new_amount;
		is_last
	}
}

impl PlayerTask for DrawTask {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let hand_size = game.players[player_index].hand.num_top_cards();
		if self.cannot_draw_at_all(hand_size) {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let is_last = self.decrement_and_check_if_is_last_draw(hand_size);
		game.replentish_for(1);
		let stack = game.draw.deal();
		let card = &stack.top.to_owned();
		game.players[player_index].hand.add(stack);

		player_has_drawn(context, game, player_index, card);

		// TODO: Check everything about drawing...
		// PlayOnDraw and buffs....

		// into param is not compatible with multiple draws...
		// maybe we are re using this class for too many things?
		// nah

		if let Some(param) = self.param {
			game.players[player_index]
				.tasks
				.set_card_value(param, Some(card.id))?;
		}

		if is_last {
			Ok(TaskProgressResult::TaskComplete)
		} else {
			Ok(TaskProgressResult::ProgressMade)
		}
	}

	fn label(&self) -> String {
		format!("Draw {:?} cards.", self.amount)
	}
}
