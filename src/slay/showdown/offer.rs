use crate::slay::deadlines::get_offer_challenges_deadline;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state::Player;

use crate::slay::choices::{Choice, ChoiceLocator, Choices};
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::specification::CardType;

use crate::slay::showdown::base::ShowDown;
use crate::slay::showdown::challenge::ChallengeState;
use crate::slay::showdown::completion::CompletionTracker;
use crate::slay::showdown::consequences::RollConsequences;

use crate::slay::showdown::roll_choices::{ChallengeChoice, SetComplete};

use super::common::{ChallengeReason, ModificationPath, RollModification};

#[derive(Debug, Clone)]
pub struct OfferChallengesState {
    player_index: usize,
    reason: ChallengeReason,
    pub completion_tracker: CompletionTracker,
    consequences: RollConsequences,
    number_of_players: usize,
}

impl OfferChallengesState {
    pub fn new(
        number_of_players: usize,
        player_index: usize,
        consequences: RollConsequences,
        reason: ChallengeReason,
    ) -> Self {
        Self {
            player_index,
            completion_tracker: CompletionTracker::new(
                number_of_players,
                get_offer_challenges_deadline(),
            ),
            consequences,
            number_of_players,
            reason,
        }
    }

    fn list_challenge_choices(
        &self,
        context: &mut GameBookKeeping,
        challenging_player: &Player,
        default_choice: u32,
    ) -> Vec<Box<dyn Choice>> {
        let mut ret = vec![Box::new(SetComplete::all_done(ChoiceLocator {
            id: default_choice,
            player_index: challenging_player.player_index,
        })) as Box<dyn Choice>];
        ret.extend(
            challenging_player
                .hand
                .list_top_cards_by_type(&CardType::Challenge)
                .iter()
                .map(|card_id| {
                    Box::new(ChallengeChoice::new(
                        *card_id,
                        ChoiceLocator {
                            id: context.id_generator.generate(),
                            player_index: challenging_player.player_index,
                        },
                    )) as Box<dyn Choice>
                }),
        );
        ret
    }

    pub fn to_challenge(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        challenger_index: usize,
    ) -> SlayResult<ChallengeState> {
        Ok(ChallengeState::new(
            rng,
            self.number_of_players,
            self.player_index,
            challenger_index,
            self.consequences.to_owned(), // Copied, although it is about to be dropped.
            self.reason.to_owned(),
        ))
    }
}

impl ShowDown for OfferChallengesState {
    fn tracker(&self) -> &CompletionTracker {
        &self.completion_tracker
    }

    fn tracker_mut(&mut self) -> &mut CompletionTracker {
        &mut self.completion_tracker
    }

    fn create_choice_for(
        &self,
        context: &mut crate::slay::game_context::GameBookKeeping,
        player: &Player,
    ) -> Choices {
        let default_choice = context.id_generator.generate();
        Choices {
            instructions: "Choose whether to modify the current challenge.".to_string(),
            default_choice,
            deadline: self.tracker().deadline,
            options: self.list_challenge_choices(context, player, default_choice),
        }
    }

    fn finish(
        &mut self,
        context: &mut crate::slay::game_context::GameBookKeeping,
        game: &mut crate::slay::state::Game,
    ) {
        self.consequences.proceed(context, game);
    }
}
