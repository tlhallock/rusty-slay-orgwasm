use crate::slay::choices::Choices;
use crate::slay::errors::SlayError;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::state;
use crate::slay::state::Game;


use std::collections::HashMap;
use std::fmt::Debug;

use super::challenge::ChallengeState;
use super::common::ModificationPath;
use super::common::RollModification;
use super::completion::CompletionTracker;
use super::offer::OfferChallengesState;
use super::roll_state::RollState;

dyn_clone::clone_trait_object!(ShowDown);

#[derive(Clone, Debug, PartialEq, Eq)]
enum ShowDownType {
	None,
	Roll,
	OfferChallenges,
	Challenge,
}

#[derive(Clone, Debug)]
pub struct CurrentShowdown {
	show_down_type: ShowDownType,

	roll: Option<RollState>,
	offer: Option<OfferChallengesState>,
	challenge: Option<ChallengeState>,
}

impl Default for CurrentShowdown {
	fn default() -> Self {
		Self {
			show_down_type: ShowDownType::None,
			roll: Default::default(),
			offer: Default::default(),
			challenge: Default::default(),
		}
	}
}

pub struct ModificationTask {
	choices_to_assign: HashMap<usize, Choices>,
}

impl ModificationTask {
	pub fn apply(&self, _context: &mut GameBookKeeping, game: &mut Game) {
		for (player_index, choices) in self.choices_to_assign.iter() {
			game.players[*player_index].choices = Some(choices.to_owned());
		}
	}
}

impl CurrentShowdown {
	// pub fn set_showdown(
	//     &mut self,
	//     context: &mut game_context::GameBookKeeping,
	//     showdown: &mut Option<Box<dyn ShowDown>>,
	// ) {
	//     showdown
	//         .as_ref()
	//         .map(|s| s.assign_all_choices(context, self));
	//     self.showdown = showdown.take();
	// }

	pub fn get_roll(&self) -> Option<&RollState> {
		self.roll.as_ref()
	}

	pub fn take_current_offer(&mut self) -> SlayResult<OfferChallengesState> {
		if self.show_down_type != ShowDownType::OfferChallenges {
			return Err(SlayError::new(
				"Needed to be offering challenges, instead there was no roll event.",
			));
		}
		self
			.offer
			.take()
			.ok_or_else(|| SlayError::new("No eyes shall see this"))
	}

	pub fn roll(&mut self, roll: RollState) {
		if self.show_down_type != ShowDownType::None {
			panic!();
		}
		self.show_down_type = ShowDownType::Roll;
		self.roll = Some(roll);
		self.challenge = None;
		self.offer = None;
	}

	pub fn challenge(&mut self, challenge: ChallengeState) {
		if self.show_down_type != ShowDownType::None {
			panic!();
		}
		self.show_down_type = ShowDownType::Challenge;
		self.roll = None;
		self.challenge = Some(challenge);
		self.offer = None;
	}

	pub fn offer(&mut self, offer: OfferChallengesState) {
		if self.show_down_type != ShowDownType::None {
			panic!();
		}
		self.show_down_type = ShowDownType::OfferChallenges;
		self.roll = None;
		self.challenge = None;
		self.offer = Some(offer);
	}
	pub fn clear(&mut self) {
		if self.show_down_type == ShowDownType::None {
			panic!();
		}
		self.roll = None;
		self.challenge = None;
		self.offer = None;
	}

	pub fn current(&self) -> Option<&dyn ShowDown> {
		match self.show_down_type {
			ShowDownType::None => None,
			ShowDownType::Roll => self.roll.as_ref().map(|x| x as &dyn ShowDown),
			ShowDownType::OfferChallenges => self.offer.as_ref().map(|x| x as &dyn ShowDown),
			ShowDownType::Challenge => self.challenge.as_ref().map(|x| x as &dyn ShowDown),
		}
	}
	pub fn current_mut(&mut self) -> Option<&mut dyn ShowDown> {
		match self.show_down_type {
			ShowDownType::None => None,
			ShowDownType::Roll => self.roll.as_mut().map(|x| x as &mut dyn ShowDown),
			ShowDownType::OfferChallenges => self.offer.as_mut().map(|x| x as &mut dyn ShowDown),
			ShowDownType::Challenge => self.challenge.as_mut().map(|x| x as &mut dyn ShowDown),
		}
	}
	// pub fn current_mut(&mut self) -> Option<Box<dyn ShowDown>> {
	//     match self.show_down_type {
	//         ShowDownType::None => None,
	//         ShowDownType::Roll => self.roll.map(|x| Box::new(x) as Box::<dyn ShowDown>),
	//         ShowDownType::OfferChallenges => self.offer.map(|x| Box::new(x) as Box::<dyn ShowDown>),
	//         ShowDownType::Challenge => self.challenge.map(|x| Box::new(x) as Box::<dyn ShowDown>),
	//     }
	// }

	// fn assign_all_choices(
	//     &mut self,
	//     context: &mut GameBookKeeping,
	//     game: &mut Game,
	// ) {
	//     self.current_mut().map(
	//         |s| s.assign_all_choices(context, game);
	//     );
	// }

	pub fn take_complete(&mut self) -> Option<Box<dyn ShowDown>> {
		if !self.current().iter().any(|s| s.tracker().is_complete()) {
			return None;
		}
		match self.show_down_type {
			ShowDownType::None => panic!(),
			ShowDownType::Roll => self.roll.take().map(|x| Box::new(x) as Box<dyn ShowDown>),
			ShowDownType::OfferChallenges => self.offer.take().map(|x| Box::new(x) as Box<dyn ShowDown>),
			ShowDownType::Challenge => self
				.challenge
				.take()
				.map(|x| Box::new(x) as Box<dyn ShowDown>),
		}
	}

	pub(crate) fn add_modification(
		&mut self, modification_path: ModificationPath, modification: RollModification,
	) -> SlayResult<()> {
		match modification_path {
			ModificationPath::Roll => match self.show_down_type {
				ShowDownType::None => return Err(SlayError::new("ASFDGAJKLRIYU")),
				ShowDownType::Roll => {}
				ShowDownType::OfferChallenges => return Err(SlayError::new("ADSHAF")),
				ShowDownType::Challenge => return Err(SlayError::new("adsfbHJJJQ")),
			},
			ModificationPath::Challenger => match self.show_down_type {
				ShowDownType::None => return Err(SlayError::new("asdfgaq4hadf")),
				ShowDownType::Roll => return Err(SlayError::new("aevq34yq")),
				ShowDownType::OfferChallenges => return Err(SlayError::new("3vaesdghar")),
				ShowDownType::Challenge => {}
			},
			ModificationPath::Initiator => match self.show_down_type {
				ShowDownType::None => return Err(SlayError::new("gadsgadg")),
				ShowDownType::Roll => return Err(SlayError::new("mp[aqgjsy")),
				ShowDownType::OfferChallenges => return Err(SlayError::new("")),
				ShowDownType::Challenge => {}
			},
		}

		match self.show_down_type {
			ShowDownType::None => panic!(),
			ShowDownType::OfferChallenges => panic!(),
			ShowDownType::Roll => self.roll.as_mut().unwrap().add_modification(modification),
			ShowDownType::Challenge => self
				.challenge
				.as_mut()
				.unwrap()
				.add_modification(modification_path, modification),
		};
		Ok(())
	}

	pub(crate) fn get_modification_task(
		&self, context: &mut GameBookKeeping, game: &Game, modifying_player_index: usize,
	) -> ModificationTask {
		let current = self.current().unwrap();
		ModificationTask {
			choices_to_assign: (0..game.number_of_players())
				.filter(|player_index| {
					*player_index == modifying_player_index
						|| current
							.tracker()
							.should_offer_modifications_again(*player_index)
				})
				.map(|player_index| {
					(
						player_index,
						current.create_choice_for(context, game, player_index),
					)
				})
				.collect(),
		}
	}

	pub(crate) fn set_player_completion(
		&mut self, player_index: usize, persist: super::completion::RollCompletion,
	) -> SlayResult<()> {
		match self.show_down_type {
			ShowDownType::None => return Err(SlayError::new("alskjdf;alksjdf;")),
			ShowDownType::Roll => self.roll.as_mut().map(|x| {
				x.completion_tracker
					.set_player_completion(player_index, persist)
			}),
			ShowDownType::OfferChallenges => self.offer.as_mut().map(|x| {
				x.completion_tracker
					.set_player_completion(player_index, persist)
			}),
			ShowDownType::Challenge => self.challenge.as_mut().map(|x| {
				x.completion_tracker
					.set_player_completion(player_index, persist)
			}),
		};
		Ok(())
	}
}

pub trait ShowDown: Debug + dyn_clone::DynClone {
	fn tracker(&self) -> &CompletionTracker;
	fn tracker_mut(&mut self) -> &mut CompletionTracker;

	// fn add_modification(
	//     &mut self,
	//     modification: RollModification,
	// );

	fn create_choice_for(
		&self, context: &mut GameBookKeeping, game: &Game, player_index: usize,
	) -> Choices;

	fn finish(&mut self, _context: &mut game_context::GameBookKeeping, game: &mut state::Game);

	fn assign_all_choices(
		&self, context: &mut game_context::GameBookKeeping, game: &mut state::Game,
	) {
		let nb_players = game.number_of_players();
		for player_index in 0..nb_players {
			let choices = Some(self.create_choice_for(context, game, player_index));
			game.players[player_index].choices = choices;
		}
	}

	// fn assign_unpersisted_choices(
	//     &self,
	//     context: &mut game_context::GameBookKeeping,
	//     game: &mut state::Game,
	// ) {
	//     game.players
	//         .iter_mut()
	//         .for_each(|player| self.assign_choices(context, player));
	// }
}
