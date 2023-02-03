use crate::slay::choices::Action;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::TasksChoice;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::notification::Notification;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::specs::items::AnotherItemType;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::move_card::MoveCardTask;
use crate::slay::tasks::tasks::offer_challenges::OfferChallengesTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

pub fn create_place_item_task(players_with_stacks: Vec<ids::PlayerIndex>) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			ChoosePlayerParameterTask::one_of(TaskParamName::PlayerToGiveItem, players_with_stacks),
			// TODO
			ClearParamsTask::create(),
		],
	})
}

pub fn create_place_item_challenge_offer(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
) -> Option<Box<dyn PlayerTask>> {
	let players_with_stacks = game.players_with_stacks();
	if players_with_stacks.is_empty() {
		log::info!("There are no places to put the item.");
		context.emit(&Notification::NoWhereToPlaceItem);
		return None;
	}
	let place_item = create_place_item_task(players_with_stacks);
	if game.players[player_index].has_modifier(PlayerModifier::ItemsCannotBeChallenged) {
		return Some(place_item);
	}
	Some(Box::new(OfferChallengesTask::new(OfferChallengesState::new(
		player_index,
		RollConsequences {
			success: RollConsequence {
				condition: Condition::challenge_denied(),
				tasks: vec![place_item],
			},
			loss: Some(RollConsequence {
				condition: Condition::challenge_sustained(),
				tasks: vec![Box::new(MoveCardTask {
					source: DeckPath::Hand(player_index),
					destination: DeckPath::Discard,
					card_id: card.id,
				}) as Box<dyn PlayerTask>],
			}),
		},
		ChallengeReason::PlaceHeroCard(card.card_type),
	))) as Box<dyn PlayerTask>)
}

pub fn create_place_item_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	placer_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	card: &Card,
	display_type: ChoiceDisplayType,
	item_card: AnotherItemType,
) -> Option<TasksChoice> {
	create_place_item_challenge_offer(context, game, placer_index, card).map(|challenge_offer| {
		TasksChoice::new(
			id,
			Choice::UseActionPoints(Action::PlaceItem(item_card)),
			display_type,
			vec![Box::new(RemoveActionPointsTask::new(1)), challenge_offer],
		)
	})
}
