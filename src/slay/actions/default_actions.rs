use crate::slay::choices::Action;
use crate::slay::choices::Choice;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::DisplayPath;
use crate::slay::choices::TasksChoice;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;
use crate::slay::tasks::tasks::replace_hand::ReplaceHandTask;

pub fn create_draw_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::Draw),
		ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Draw)),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			DrawTask::create(1),
		],
	)
}

pub fn create_replace_hand_choice(id: ids::ChoiceId) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::ReplaceHand),
		ChoiceDisplayType::HighlightPath(DisplayPath::DeckAt(DeckPath::Discard)),
		vec![
			Box::new(RemoveActionPointsTask::new(3)),
			Box::new(ReplaceHandTask {}),
		],
	)
}

pub fn create_forfeit_choice(
	game: &mut Game,
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
) -> TasksChoice {
	let current_amount_remaining = game.players[player_index].get_remaining_action_points();
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::Forfeit),
		ChoiceDisplayType::Forfeit,
		vec![Box::new(RemoveActionPointsTask::new(current_amount_remaining)) as Box<dyn PlayerTask>],
	)
}
