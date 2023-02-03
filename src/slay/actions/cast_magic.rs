use crate::slay::choices::Action;
use crate::slay::choices::CardPath;
use crate::slay::choices::Choice;
use crate::slay::choices::TasksChoice;
use crate::slay::ids;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::showdown::offer::OfferChallengesState;
use crate::slay::showdown::roll::ChallengeReason;
use crate::slay::specs::magic::MagicSpell;
use crate::slay::state::game::Game;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::magic::MagicTask;
use crate::slay::tasks::tasks::offer_challenges::OfferChallengesTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

pub fn create_cast_magic_choice(
	game: &Game,
	player_index: ids::PlayerIndex,
	id: ids::ChoiceId,
	card_path: CardPath,
	spell: MagicSpell,
) -> TasksChoice {
	TasksChoice::new(
		id,
		Choice::UseActionPoints(Action::CastMagic(spell)),
		card_path.display().to_highlight(),
		vec![
			Box::new(RemoveActionPointsTask::new(1)),
			card_path.get_discard_task(),
			Box::new(OfferChallengesTask::new(OfferChallengesState::new(
				player_index,
				RollConsequences {
					success: RollConsequence {
						condition: Condition::challenge_denied(),
						tasks: vec![Box::new(MagicTask::new(spell)) as Box<dyn PlayerTask>],
					},
					loss: None,
				},
				ChallengeReason::CastMagic(game.card(card_path).card_type),
			))) as Box<dyn PlayerTask>,
		],
	)
}
