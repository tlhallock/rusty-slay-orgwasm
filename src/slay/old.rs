// // lol, I can't say `use crate::ids;`

// use super::choices::DisplayPath;
// // use super::ids::{CardId, ChallengeId, ChoiceId, DeckId, ElementId, IdGenerator, PlayerId, RollId};
// use super::ids;
// use super::modifiers::PlayerBuffs;
// use super::specification;
// use super::specification::CardSpec;
// use super::specification::CardType;
// use super::tasks::PlayerTask;
// use super::tasks::PlayerTasks;
// use crate::slay::choices;
// use crate::slay::errors;
// use crate::slay::game_context;
// use crate::slay::modifiers;
// use crate::slay::showdown::current_showdown::CurrentShowdown;
// use std::io::Write;

// use crate::slay::specification::HeroType;
// use crate::slay::tasks;

// use errors::SlayResult;

// use std::collections::HashSet;
// use std::collections::VecDeque;

// use std::fmt::Debug;

// use std::io::BufWriter;
// use std::ops::RangeBounds;

// use std::iter::Iterator;

// #[derive(Debug, Clone, Copy)]
// pub enum ChoiceParamType {
// 	Player,
// 	Card,
// 	Enumeration,
// 	Index,
// }


// // Split into choice type?
// // #[derive(Debug, Clone)]
// // pub enum Action {
// //     ReplaceHand,
// //     DrawCard,
// //     Forfeit,

// //     Attack(ids::CardId),
// //     PlaceCard(ids::CardId),
// //     CastMagic(ids::CardId),
// //     UseHero(ids::CardId),
// // }


