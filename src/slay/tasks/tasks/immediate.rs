use crate::slay::actions;
use crate::slay::actions::create_place_hero_challenges;
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
use crate::slay::notification::Notification;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::task_params::TaskParamName;

pub fn play_card_from_hand(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
	card: &Card,
) {
	// TODO: refactor this into a player_from_hand...
	let mut play_immediately_tasks = Vec::new();
	if let Some(_item_modifier) = card.get_spec().card_modifier.as_ref() {
		// This card is an item card...
		// src/slay/actions.rs:452
		let players_with_stacks = game.players_with_stacks();
		if players_with_stacks.is_empty() {
			log::info!("There are no places to put the item.");
			context.emit(&Notification::NoWhereToPlaceItem);
			return;
		}
		play_immediately_tasks.push(actions::create_place_item_challenge_offer(
			game,
			player_index,
			card,
			players_with_stacks,
		));
	} else if let SlayCardSpec::HeroCard(hero_card) = card.card_type {
		// This card is a hero card...
		play_immediately_tasks.push(create_place_hero_challenges(
			context,
			game,
			player_index,
			CardPath::TopCardIn(DeckPath::Hand(player_index), card.id),
			hero_card,
		));
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
}

impl PlayImmediatelyFilter {
	pub fn can_play_immediately(&self, card: &Card) -> bool {
		match self {
			PlayImmediatelyFilter::IsMagic => card.is_magic(),
			PlayImmediatelyFilter::IsHero => card.is_hero(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct OfferPlayImmediately {
	card_param: TaskParamName,
	filter: PlayImmediatelyFilter,
}

impl OfferPlayImmediately {
	pub fn create(card_param: TaskParamName, filter: PlayImmediatelyFilter) -> Box<dyn PlayerTask> {
		Box::new(OfferPlayImmediately { card_param, filter })
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
		play_card_from_hand(context, game, player_index, &card);
		Ok(TaskProgressResult::TaskComplete)
	}
	fn label(&self) -> String {
		"Offer to play a card immediately".to_owned()
	}
}
