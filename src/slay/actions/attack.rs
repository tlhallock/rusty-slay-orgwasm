use crate::slay::choices::Action;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::TasksChoice;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::showdown::roll_state::RollState;
use crate::slay::specs::monster::Monster;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::do_roll::DoRollTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

pub fn create_attack_monster_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	monster: Monster,
) -> TasksChoice {
	TasksChoice::new(
		context.id_generator.generate(),
		Choice::UseActionPoints(Action::AttackMonster(monster)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(2)) as Box<dyn PlayerTask>,
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				monster.get_consequences(card_path.get_card_id()),
				RollReason::AttackMonster(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}
