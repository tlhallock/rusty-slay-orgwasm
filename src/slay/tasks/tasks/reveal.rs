use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;

#[derive(Clone, Debug)]
pub struct Reveal {
	spec: SlayCardSpec,
}

impl Reveal {
	pub fn create(spec: SlayCardSpec) -> Box<dyn PlayerTask> {
		Box::new(Self { spec })
	}
}

impl PlayerTask for Reveal {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		context.emit(&Notification::PlayerDrew(player_index, self.spec));
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		"Reveal a drawn card.".to_owned()
	}
}
