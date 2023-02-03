use crate::slay::choices::Action;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::TasksChoice;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specs::hero::HeroAbilityType;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::card_used::CardUsedTask;
use crate::slay::tasks::tasks::do_roll::DoRollTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

pub fn create_roll_for_ability_task(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	hero_card: HeroAbilityType,
) -> Box<dyn PlayerTask> {
	Box::new(AddTasks {
		tasks: vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				hero_card.to_consequences(),
				RollReason::UseHeroAbility(card.card_type),
			))) as Box<dyn PlayerTask>,
		],
	})
}

pub fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::RollForAbility(hero_card)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			create_roll_for_ability_task(context, game, player_index, game.card(card_path), hero_card),
		],
	)
}
