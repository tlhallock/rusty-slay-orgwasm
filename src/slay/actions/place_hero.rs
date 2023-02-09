use std::iter;

use crate::slay::choices::Action;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::TasksChoice;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::offer_challenges::OfferChallengesTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

use super::cast_magic::cannot_be_challenged;
use super::roll_for_ability;

pub fn create_place_hero_challenges(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> Vec<Box<dyn PlayerTask>> {
	let roll = roll_for_ability::create_roll_for_ability_task(
		context,
		game,
		player_index,
		game.card(card_path),
		hero_card,
	);
	let tasks = iter::once(Some(card_path.get_place_task()))
		.chain(iter::once(roll))
		.flatten()
		.collect();
	if cannot_be_challenged(game, player_index) {
		return tasks;
	}
	vec![Box::new(OfferChallengesTask::new(OfferChallengesState::new(
		player_index,
		RollConsequences {
			success: RollConsequence {
				condition: Condition::challenge_denied(),
				tasks,
			},
			loss: Some(RollConsequence {
				condition: Condition::challenge_sustained(),
				tasks: vec![card_path.get_discard_task()],
			}),
		},
		ChallengeReason::PlaceHeroCard(game.card(card_path).card_type),
	))) as Box<dyn PlayerTask>]
}

pub fn create_place_hero_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::PlaceHeroInParty(hero_card)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			AddTasks::create(create_place_hero_challenges(
				context,
				game,
				player_index,
				card_path,
				hero_card,
			)),
		],
	)
}
