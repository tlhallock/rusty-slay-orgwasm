use crate::slay::choices::{
    self, ChoiceDisplay, ChoiceInformation, ChoiceLocator, DisplayArrow, DisplayPath,
};
use crate::slay::errors::{SlayError, SlayResult};
use crate::slay::ids;
use crate::slay::state::{self, DeckPath};
use crate::slay::{game_context, state_modifiers};

use super::base::ShowDown;
use super::common::{ModificationPath, RollModification};
use super::completion::RollCompletion;

#[derive(Clone, Debug)]
pub struct ModifyRollChoice {
    modification: Option<RollModification>,
    choice_information: choices::ChoiceInformation,
    modification_path: ModificationPath,
}

impl ModifyRollChoice {
    pub fn new(
        modification: RollModification,
        choice_information: choices::ChoiceInformation,
        path: ModificationPath,
    ) -> Self {
        Self {
            modification: Some(modification),
            choice_information,
            modification_path: path,
        }
    }

    pub fn new2(
        modification: RollModification,
        locator: choices::ChoiceLocator,
        modification_path: ModificationPath,
    ) -> Self {
        let modifying_player_index = modification.modifying_player_index;
        let card_id = modification.card_id;
        let modification_amount = modification.modification_amount;

        Self {
            modification_path,
            modification: Some(modification),
            choice_information: choices::ChoiceInformation {
                locator,
                // locator: choices::ChoiceLocator {
                //     id: context.id_generator.generate(),
                //     player_index,
                // },
                display: choices::ChoiceDisplay {
                    highlight: Some(choices::DisplayPath::CardIn(
                        state::DeckPath::Hand(modifying_player_index),
                        card_id,
                    )),
                    arrows: vec![
                        choices::DisplayArrow {
                            source: choices::DisplayPath::CardIn(
                                state::DeckPath::Hand(modifying_player_index),
                                card_id,
                            ),
                            destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
                        },
                        choices::DisplayArrow {
                            source: DisplayPath::CardIn(
                                DeckPath::Hand(modifying_player_index),
                                card_id,
                            ),
                            destination: DisplayPath::Roll(modification_path),
                        },
                    ],
                    label: format!(
                        "Use {} to modify {}'s roll by {}",
                        card_id, "somebody", modification_amount,
                    ),
                },
            },
        }
    }
}

impl choices::Choice for ModifyRollChoice {
    fn select(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<()> {
        let modification = self
            .modification
            .take()
            .ok_or_else(|| SlayError::new("Cannot choose the same choice twice."))?;
        state_modifiers::transfer_a_top_card(
            modification.card_id,
            &mut game.players[modification.modifying_player_index].hand,
            &mut game.draw,
        )?;

        let modifying_player_index = modification.modifying_player_index;
        game.showdown
            .add_modification(self.modification_path, modification)?;
        let modification_task =
            game.showdown
                .get_modification_task(context, game, modifying_player_index);
        modification_task.apply(context, game);
        Ok(())
    }

    fn get_choice_information(&self) -> &choices::ChoiceInformation {
        &self.choice_information
    }
}

#[derive(Clone, Debug)]
pub struct SetComplete {
    persist: RollCompletion,
    choice_information: choices::ChoiceInformation,
}

impl SetComplete {
    pub fn until_modification(locator: choices::ChoiceLocator) -> Self {
        Self {
            persist: RollCompletion::DoneUntilModification,
            choice_information: choices::ChoiceInformation {
                locator,
                display: choices::ChoiceDisplay {
                    highlight: None,
                    label: "Don't modify this roll unless someone else does.".to_string(),
                    ..Default::default()
                },
            },
        }
    }

    pub fn all_done(locator: choices::ChoiceLocator) -> Self {
        Self {
            persist: RollCompletion::AllDone,
            choice_information: choices::ChoiceInformation {
                locator,
                display: choices::ChoiceDisplay {
                    highlight: None,
                    label: "Do not modify this roll.".to_string(),
                    ..Default::default()
                },
            },
        }
    }
}

impl choices::Choice for SetComplete {
    fn select(
        &mut self,
        _context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<()> {
        let player_index = self.choice_information.player_index();
        game.showdown
            .set_player_completion(player_index, self.persist)?;
        // .as_mut()
        // .ok_or_else(|| SlayError::new("No show down"))?
        // .tracker_mut()
        // .set_player_completion(player_index, self.persist);
        game.players[player_index].choices = None;
        Ok(())
    }

    fn get_choice_information(&self) -> &choices::ChoiceInformation {
        &self.choice_information
    }
}

#[derive(Debug, Clone)]
pub struct ChallengeChoice {
    challenge_card_id: ids::CardId,
    choice_information: ChoiceInformation,
}

impl ChallengeChoice {
    pub fn new(challenge_card_id: ids::CardId, locator: ChoiceLocator) -> Self {
        let player_index = locator.player_index;
        Self {
            challenge_card_id,
            choice_information: ChoiceInformation {
                locator,
                display: ChoiceDisplay {
                    highlight: Some(DisplayPath::CardIn(
                        DeckPath::Hand(player_index),
                        challenge_card_id,
                    )),
                    arrows: vec![DisplayArrow {
                        source: DisplayPath::CardIn(
                            DeckPath::Hand(player_index),
                            challenge_card_id,
                        ),
                        destination: DisplayPath::DeckAt(DeckPath::Discard),
                    }],
                    label: "Challenge that action...".to_string(),
                },
            },
        }
    }
}

impl choices::Choice for ChallengeChoice {
    fn select(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<()> {
        let challenging_player_index = self.choice_information.locator.player_index;
        game.move_card(
            state::DeckPath::Hand(challenging_player_index),
            state::DeckPath::Discard,
            self.challenge_card_id,
        )?;
        let offer = game.showdown.take_current_offer()?;
        let mut challenge = offer.to_challenge(&mut context.rng, challenging_player_index)?;
        challenge.assign_all_choices(context, game);
        game.showdown.challenge(challenge);
        Ok(())
    }

    fn get_choice_information(&self) -> &choices::ChoiceInformation {
        &self.choice_information
    }
}
