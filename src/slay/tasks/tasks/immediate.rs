use crate::slay::actions;
use crate::slay::actions::create_place_hero_challenges;
use crate::slay::actions::create_roll_for_ability_task;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;



pub fn create_play_card_immediately_task(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
) -> Option<Box<dyn PlayerTask>> {
	match card.card_type {
    SlayCardSpec::HeroCard(hero_card) => {
			let hand_path = CardPath::TopCardIn(DeckPath::Hand(player_index), card.id);
			let party_path = CardPath::TopCardIn(DeckPath::Party(player_index), card.id);
			if let Some(_) = game.maybe_card(hand_path) {
				Some(create_place_hero_challenges(
					context,
					game,
					player_index,
					hand_path,
					hero_card,
				))
			} else if let Some(_) = game.maybe_card(party_path) {
				Some(create_roll_for_ability_task(
					context,
					game,
					player_index,
					card,
					hero_card,
				))
			} else {
				unreachable!()
			}
		},
    SlayCardSpec::MagicCard(_) => todo!(),
    SlayCardSpec::Item(_) => actions::create_place_item_challenge_offer(
			context,
			game,
			player_index,
			card,
		),
		SlayCardSpec::ModifierCard(_)
    | SlayCardSpec::PartyLeader(_)
    | SlayCardSpec::MonsterCard(_)
    | SlayCardSpec::Challenge => None, // unreachable!(),
	}
}

// This should be a task...
pub fn play_card_immediately(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	mut extra_task: Option<Box<dyn PlayerTask>>,
) {
	let mut play_immediately_tasks = Vec::new();
	if let Some(task) = create_play_card_immediately_task(
		context,
		game,
		player_index,
		card,
	) {
		play_immediately_tasks.push(task);
	}

	if let Some(task) = extra_task.take() {
		play_immediately_tasks.push(task);
	}

	let default_choice = context.id_generator.generate();
	game.players[player_index].choices = Some(Choices {
		choices_type: ChoicesType::PlayImmediately(card.card_type),
		timeline: deadlines::get_refactor_me_deadline(),
		default_choice: Some(default_choice),
		options: vec![
			TasksChoice::prepend(
				context.id_generator.generate(),
				Choice::PlayImmediately(card.card_type),
				ChoiceDisplayType::Yes,
				play_immediately_tasks,
			),
			TasksChoice::prepend(
				default_choice,
				Choice::DoNotPlayImmediately,
				ChoiceDisplayType::No,
				vec![],
			),
		],
	});
}

#[derive(Clone, Debug)]
pub enum PlayImmediatelyFilter {
	IsMagic,
	IsHero,
	None,
	IsChallenge,
	IsItem,
}

impl PlayImmediatelyFilter {
	pub fn can_play_immediately(&self, card: &Card) -> bool {
		match self {
			PlayImmediatelyFilter::IsMagic => card.is_magic(),
			PlayImmediatelyFilter::IsHero => card.is_hero(),
			PlayImmediatelyFilter::IsChallenge => card.is_challenge(),
			PlayImmediatelyFilter::IsItem => card.is_item(),
    	PlayImmediatelyFilter::None => true,
		}
	}
}

#[derive(Clone, Debug)]
pub struct OfferPlayImmediately {
	card_param: TaskParamName,
	filter: PlayImmediatelyFilter,
	extra_task: Option<Box<dyn PlayerTask>>,
}

impl OfferPlayImmediately {
	pub fn create(card_param: TaskParamName, filter: PlayImmediatelyFilter) -> Box<dyn PlayerTask> {
		Box::new(OfferPlayImmediately { card_param, filter, extra_task: None, })
	}

	pub fn with_an_extra_task(
		card_param: TaskParamName,
		filter: PlayImmediatelyFilter,
		extra_task: Box<dyn PlayerTask>,
	) -> Box<dyn PlayerTask> {
		Box::new(OfferPlayImmediately { card_param, filter, extra_task: Some(extra_task) })
	}
}

impl PlayerTask for OfferPlayImmediately {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		let card_id = game.card_param(player_index, &self.card_param)?;
		if card_id.is_none() {
			return Ok(TaskProgressResult::TaskComplete);
		}
		let card_id = card_id.unwrap();
		let card = game
			// .deck(deck_path)
			.find_card(card_id)
			.ok_or_else(|| SlayError::new("Unable to find card."))?
			.to_owned();

		if !self.filter.can_play_immediately(&card) {
			return Ok(TaskProgressResult::TaskComplete);
		}
		play_card_immediately(context, game, player_index, &card, self.extra_task.take());
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Offer to play a card immediately".to_owned()
	}
}
