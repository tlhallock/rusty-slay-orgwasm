use crate::slay::choices;
use crate::slay::choices::Choices;
use crate::slay::deadlines;
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::specification::CardSpec;
use crate::slay::state;
use crate::slay::state::Player;

use crate::slay::showdown::roll_choices::ModifyRollChoice;

use super::base::ShowDown;
use super::challenge::ChallengeState;
use super::common::ModificationPath;
use super::common::Roll;
use super::common::RollModification;
use super::completion::CompletionTracker;
use super::consequences::RollConsequences;
use super::roll_choices::SetComplete;

// Only the party needs stacks...

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum RollReason {
    UseHeroAbility(CardSpec),
    AttackMonster(CardSpec),
}

#[derive(Debug, Clone)]
pub struct RollState {
    pub roller_index: usize,
    pub reason: RollReason,
    consequences: RollConsequences,
    pub initial: Roll,
    pub history: Vec<RollModification>,
    pub completion_tracker: CompletionTracker,
}

impl RollState {
    pub fn new(
        roller_index: usize,
        consequences: RollConsequences,
        initial: Roll,
        num_players: usize,
        reason: RollReason,
    ) -> Self {
        Self {
            roller_index,
            initial,
            history: Default::default(),
            consequences,
            completion_tracker: CompletionTracker::new(num_players, deadlines::get_roll_deadline()),
            reason,
        }
    }

    pub fn add_modification(&mut self, modification: RollModification) {
        self.tracker_mut().deadline = deadlines::get_roll_deadline();
        self.history.push(modification);
    }

    pub fn calculate_roll_total(&self) -> i32 {
        self.initial.die1 as i32
            + self.initial.die2 as i32
            + self
                .history
                .iter()
                .map(|h| h.modification_amount)
                .sum::<i32>()
    }
    // fn set_player_completion(&mut self, player_index: usize, persist: RollCompletion) {
    //     self.completion_tracker.set_player_completion(player_index, persist);
    // }

    // fn should_offer_modifications_again(&self, player_index: usize) -> bool {
    //     self.completion_tracker.should_offer_modifications_again(player_index)
    // }
}

impl ShowDown for RollState {
    fn tracker(&self) -> &CompletionTracker {
        &self.completion_tracker
    }

    fn tracker_mut(&mut self) -> &mut CompletionTracker {
        &mut self.completion_tracker
    }

    fn create_choice_for(
        &self,
        context: &mut game_context::GameBookKeeping,
        player: &state::Player,
    ) -> Choices {
        let default_choice = context.id_generator.generate();
        choices::Choices {
            instructions: "Choose whether to modify the current roll.".to_string(),
            default_choice,
            deadline: self.tracker().deadline,
            options: list_modification_choices(
                context,
                player,
                default_choice,
                vec![ModificationPath::Roll],
            ),
        }
    }

    fn finish(&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game) {
        let roll_sum = self.calculate_roll_total();
        self.consequences.apply_roll_sum(game, roll_sum);
        // game.players[roll.roller_index].tasks = Some(roll.consequences.take_tasks(roll_sum));
    }
}

pub fn list_modification_choices(
    context: &mut game_context::GameBookKeeping,
    player: &Player,
    default_choice: ids::ChoiceId,
    rolls: Vec<ModificationPath>,
) -> Vec<Box<dyn choices::Choice>> {
    let player_index = player.player_index;
    let mut choices: Vec<Box<dyn choices::Choice>> = vec![
        Box::new(SetComplete::until_modification(choices::ChoiceLocator {
            id: default_choice,
            player_index,
        })),
        Box::new(SetComplete::until_modification(choices::ChoiceLocator {
            id: context.id_generator.generate(),
            player_index,
        })),
    ];

    for stack in player.hand.stacks.iter() {
        for (card_id, modification_amount) in stack.top.modification_amounts() {
            for modification_path in rolls.iter() {
                choices.push(Box::new(ModifyRollChoice::new2(
                    RollModification {
                        card_id,
                        modifying_player_index: player_index,
                        modification_amount,
                    },
                    choices::ChoiceLocator {
                        id: context.id_generator.generate(),
                        player_index,
                    },
                    *modification_path,
                )) as Box<dyn choices::Choice>)
            }
        }
    }
    // for modification_amount in

    //   choices.extend(
    //       player
    //           .hand
    //           .stacks
    //           .iter()
    //           .flat_map(|s| s.top.modification_amounts())
    //           .flat_map(|(card_id, modification_amount)| {
    //               rolls
    //                   .iter()
    //                   .map(|path| {
    //                       (
    //                           card_id,
    //                           modification_amount,
    //                           path,
    //                       )
    //                   })
    //                   .collect::<Vec<(ids::CardId, i32, ids::RollId, choices::DisplayPath)>>()
    //           })
    //           .map(|(card_id, modification_amount, roll_id, display_path)| {

    //           }),
    //   );
    choices
}

// pub fn do_roll(
//     context: &mut game_context::GameBookKeeping,
//     game: &mut state::Game,
//     roller_index: usize,
//     consequences: RollConsequences,
// ) {
//     let roll = RollState::new(
//         roller_index,
//         consequences,
//         Roll::create_from(&mut context.rng),
//         game.number_of_players(),
//     );
//     for player in game.players.iter_mut() {
//         assign_roll_choices(context, player, &roll);
//     }
//     // game.players
//     //     .iter_mut()
//     //     .for_each(|player| assign_roll_choices(context, player, &roll));
//     game.roll = Some(roll);
// }
