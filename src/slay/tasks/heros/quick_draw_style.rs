use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::immediate::create_play_card_immediately_task;
use crate::slay::tasks::tasks::immediate::PlayImmediatelyFilter;

fn add_play_immediately_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_option: Option<ids::CardId>,
	options: &mut Vec<TasksChoice>,
) {
	if let Some(card_id) = card_option {
		let card_path = CardPath::TopCardIn(DeckPath::Hand(player_index), card_id);
		let card = game.card(card_path);
		if let Some(task) = create_play_card_immediately_task(context, game, player_index, card) {
			options.push(TasksChoice::new(
				context.id_generator.generate(),
				Choice::PlayImmediately(card.card_type),
				ChoiceDisplayType::HighlightPath(DisplayPath::CardAt(card_path)),
				vec![task],
			));
		}
	}
}

#[derive(Clone, Debug)]
pub struct QuickDrawStyle {
	card_1_param: TaskParamName,
	card_2_param: TaskParamName,
	filter: PlayImmediatelyFilter,
}

impl QuickDrawStyle {
	pub fn create(
		card_1_param: TaskParamName,
		card_2_param: TaskParamName,
		filter: PlayImmediatelyFilter,
	) -> Box<dyn PlayerTask> {
		Box::new(Self {
			card_1_param,
			card_2_param,
			filter,
		}) as Box<dyn PlayerTask>
	}

	fn applies(&self, game: &Game, player_index: ids::PlayerIndex, card: Option<u32>) -> bool {
		if let Some(card_id) = card {
			self
				.filter
				.can_play_immediately(game.card(CardPath::TopCardIn(DeckPath::Hand(player_index), card_id)))
		} else {
			false
		}
	}
}

impl PlayerTask for QuickDrawStyle {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		// let mut drawn_cards = Vec::new();
		let card_1_option = game.card_param(player_index, &self.card_1_param)?;
		let card_2_option = game.card_param(player_index, &self.card_2_param)?;
		let applies = self.applies(game, player_index, card_1_option)
			|| self.applies(game, player_index, card_2_option);
		context.emit(&Notification::CanPlayImmediately(player_index, applies));
		if !applies {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let default_choice = context.id_generator.generate();
		let mut options = vec![TasksChoice::new(
			default_choice,
			Choice::DoNotPlayImmediately,
			ChoiceDisplayType::No,
			vec![],
		)];
		add_play_immediately_choice(context, game, player_index, card_1_option, &mut options);
		add_play_immediately_choice(context, game, player_index, card_2_option, &mut options);

		game.players[player_index].choose(Choices {
			choices_type: ChoicesType::PlayOneOfImmediately,
			options,
			default_choice: Some(default_choice),
			timeline: deadlines::get_refactor_me_deadline(),
		});
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"pull again".to_owned()
	}
}
