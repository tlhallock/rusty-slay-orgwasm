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
use crate::slay::status_effects::effect::HeroStatusEffect;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::tasks::add_tasks::AddTasks;
use crate::slay::tasks::tasks::card_used::CardUsedTask;
use crate::slay::tasks::tasks::do_roll::DoRollTask;
use crate::slay::tasks::tasks::remove_action_points::RemoveActionPointsTask;

// Maybe remove this task and create a RollForAbilitySuccess task?
pub struct RollForAbilityEffects {
	pub num_draw: u32,
	pub num_discard: u32,
}

impl RollForAbilityEffects {
	pub fn push_success_tasks(&self, tasks: &mut Vec<Box<dyn PlayerTask>>) {
		if self.num_draw > 0 {
			tasks.push(DrawTask::create(self.num_draw as usize));
		}
	}
	pub fn create_loss_tasks(&self) -> Option<Vec<Box<dyn PlayerTask>>> {
		if self.num_draw > 0 {
			Some(vec![DrawTask::create(self.num_draw as usize)])
		} else {
			None
		}
	}
}

pub fn create_roll_for_ability_task(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card: &Card,
	hero_card: HeroAbilityType,
) -> Option<Box<dyn PlayerTask>> {
	guard_unwrap!(
		let Some(stack) = game.players[player_index].party.stack(card.id).or_else(
			// TODO: We create the action of rolling for the ability while it is still
			// in the hand, while creating the action to place the hero...
			|| game.players[player_index].hand.stack(card.id)
		)
	);
	if stack
		.hero_effects()
		.any(|effect| matches!(effect.effect.effect, HeroStatusEffect::RemoveAbility))
	{
		return None;
	}
	Some(Box::new(AddTasks {
		tasks: vec![
			Box::new(CardUsedTask::new(player_index, card.id)),
			Box::new(DoRollTask::new(RollState::create(
				context,
				game,
				player_index,
				hero_card.to_consequences(&RollForAbilityEffects {
					num_draw: stack
						.hero_effects()
						.map(|effect| match effect.effect.effect {
							HeroStatusEffect::DrawOnUnsuccessfulRollForAbility(num) => num,
							_ => 0,
						})
						.sum::<u32>(),
					num_discard: stack
						.hero_effects()
						.map(|effect| match effect.effect.effect {
							HeroStatusEffect::DiscardOnSuccessfulRollForAbility(num) => num,
							_ => 0,
						})
						.sum::<u32>(),
				}),
				RollReason::UseHeroAbility(hero_card),
			))) as Box<dyn PlayerTask>,
		],
	}))
}

pub fn create_roll_for_ability_choice(
	context: &mut GameBookKeeping,
	game: &Game,
	player_index: ids::PlayerIndex,
	card_path: CardPath,
	hero_card: HeroAbilityType,
) -> Option<TasksChoice> {
	create_roll_for_ability_task(context, game, player_index, game.card(card_path), hero_card).map(
		|roll| {
			TasksChoice::new(
				context.id_generator.generate(),
				Choice::UseActionPoints(Action::RollForAbility(hero_card)),
				card_path.display().to_highlight(),
				vec![Box::new(RemoveActionPointsTask::new(1)), roll],
			)
		},
	)
}
