use crate::slay::choices;
use crate::slay::deadlines;
use crate::slay::errors;
use crate::slay::errors::SlayResult;
use crate::slay::game_context;
use crate::slay::ids;
use crate::slay::modifiers;
use crate::slay::state;

use core::fmt::Debug;
use dyn_clone::DynClone;
use std::collections::HashMap;
use std::collections::VecDeque;

use super::showdown::base::ShowDown;
use super::showdown::common::Roll;
use super::showdown::consequences::RollConsequenceRenameMe;
use super::showdown::consequences::RollConsequences;
use super::showdown::roll_state::RollState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskSpec {
    Sacrifice(u32),
    Discard(u32),
    ReceiveModifier(modifiers::PlayerModifier),
    Draw(u32),
}

impl TaskSpec {
    pub fn to_task(&self, player_index: usize) -> Box<dyn PlayerTask> {
        match &self {
            TaskSpec::Sacrifice(num) => Box::new(Sacrifice::new(*num, player_index)),
            TaskSpec::Discard(num) => Box::new(Discard::new(*num)),
            TaskSpec::ReceiveModifier(modifier) => Box::new(ReceiveModifier::new(*modifier)),
            TaskSpec::Draw(num) => Box::new(Draw::new(*num)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TaskParamName {
    Victim,
}

#[derive(Debug, Default, Clone)]
pub struct TaskParams {
    pub players: HashMap<TaskParamName, ids::PlayerId>,
    pub cards: HashMap<TaskParamName, ids::CardId>,
    pub index: HashMap<TaskParamName, usize>,
}

pub enum TaskProgressResult {
    TaskComplete,
    ChoicesAssigned,
}

dyn_clone::clone_trait_object!(PlayerTask);

pub trait PlayerTask: Debug + dyn_clone::DynClone {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult>;
}

// impl<'de> Deserialize<'de> for Box<dyn PlayerTask> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de> {
//         todo!()
//     }
// }

#[derive(Debug, Default, Clone)]
pub struct PlayerTasks {
    // tasks: VecDeque<PlayerTask>,
    upcoming: VecDeque<Box<dyn PlayerTask>>,
    current: Option<Box<dyn PlayerTask>>,
    params: TaskParams,
}

impl PlayerTasks {
    pub fn new(tasks: Vec<Box<dyn PlayerTask>>) -> Self {
        Self {
            upcoming: VecDeque::from(tasks),
            current: None,
            params: Default::default(),
        }
    }

    pub fn take_from(&mut self, to_take: &mut Vec<Box<dyn PlayerTask>>) {
        self.upcoming.extend(to_take.drain(..));
    }

    pub fn put_current_task_back(&mut self, task: Box<dyn PlayerTask>) -> SlayResult<()> {
        // reviewer: How do you make this a one liner?
        if self.current.is_some() {
            return Err(errors::SlayError::new(
                "The current action should be taken out right now.",
            ));
        }
        self.current = Some(task);
        Ok(())
    }
    pub fn take_current_task(&mut self) -> Option<Box<dyn PlayerTask>> {
        self.current.take().or_else(|| {
            // Initialize the task, if need be...
            self.upcoming.pop_front()
        })
    }
}

#[derive(Debug, Clone)]
struct Sacrifice {
    num: u32,
    player_index: usize,
}

impl Sacrifice {
    pub fn new(num: u32, player_index: usize) -> Self {
        Self { num, player_index }
    }
}

// TODO: This could use the move card task?
#[derive(Debug, Clone)]
struct MoveCardChoice {
    source: state::DeckPath,
    destination: state::DeckPath,
    card_id: ids::CardId,
    choice_information: choices::ChoiceInformation,
}

impl choices::Choice for MoveCardChoice {
    fn select(
        &mut self,
        _context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<()> {
        game.move_card(self.source, self.destination, self.card_id)
    }

    fn get_choice_information(&self) -> &choices::ChoiceInformation {
        &self.choice_information
    }
}

// fn card_is_sacrificable(stack: &state::Stack) -> bool {
//   true
// }

impl PlayerTask for Sacrifice {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        if self.num == 0 {
            return Ok(TaskProgressResult::TaskComplete);
        }

        let party = &game.players[self.player_index].party;
        let mut options: Vec<Box<dyn choices::Choice>> = party
            .stacks
            .iter()
            // .filter(card_is_sacrificable)
            .map(|s| {
                Box::new(MoveCardChoice {
                    source: state::DeckPath::Party(self.player_index),
                    destination: state::DeckPath::Discard,
                    card_id: s.top.id,
                    choice_information: choices::ChoiceInformation {
                        locator: choices::ChoiceLocator {
                            id: context.id_generator.generate(),
                            player_index: self.player_index,
                        },
                        display: choices::ChoiceDisplay {
                            label: format!("Sacrifice {}.", s.top.label()),
                            highlight: Some(choices::DisplayPath::CardIn(
                                state::DeckPath::Hand(self.player_index),
                                s.top.id,
                            )),
                            arrows: vec![choices::DisplayArrow {
                                source: choices::DisplayPath::CardIn(
                                    state::DeckPath::Hand(self.player_index),
                                    s.top.id,
                                ),
                                destination: choices::DisplayPath::DeckAt(state::DeckPath::Discard),
                            }],
                        },
                    },
                }) as Box<dyn choices::Choice>
            })
            .collect();

        if options.len() == self.num as usize {
            for option in options.iter_mut() {
                option.select(context, game)?;
            }
            return Ok(TaskProgressResult::TaskComplete);
        }

        if options.is_empty() {
            return Ok(TaskProgressResult::TaskComplete);
        }

        let default_choice = options[0].get_choice_information().get_id();
        game.players[self.player_index].choices = Some(choices::Choices {
            instructions: "Choose a card to sacrifice.".to_string(),
            options,
            default_choice,
            deadline: deadlines::get_sacrifice_deadline(),
        });

        self.num -= 1;
        Ok(TaskProgressResult::ChoicesAssigned)
    }
}

#[derive(Debug, Clone)]
struct ReceiveModifier {
    modifier: modifiers::PlayerModifier,
}

impl ReceiveModifier {
    pub fn new(modifier: modifiers::PlayerModifier) -> Self {
        Self { modifier }
    }
}

impl PlayerTask for ReceiveModifier {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Discard {
    num: u32,
}

impl Discard {
    pub fn new(num: u32) -> Self {
        Self { num }
    }
}

impl PlayerTask for Discard {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Draw {
    num: u32,
}

impl Draw {
    pub fn new(num: u32) -> Self {
        Self { num }
    }
}

impl PlayerTask for Draw {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        todo!()
    }
}

pub(crate) fn finish_tasks(
    context: &mut game_context::GameBookKeeping,
    game: &mut state::Game,
) -> SlayResult<bool> {
    let task_option: Option<(usize, Box<dyn PlayerTask>)> = game.take_current_task();
    if task_option.is_none() {
        return Ok(false);
    }
    if let Some((player_index, mut task)) = task_option {
        match task.make_progress(context, game)? {
            TaskProgressResult::TaskComplete => {}
            TaskProgressResult::ChoicesAssigned => {
                game.players[player_index].put_current_task_back(task)?;
                return Ok(true);
            }
        };
    }
    Ok(true)
}

#[derive(Debug, Clone)]
pub struct MoveCardTask {
    pub source: state::DeckPath,
    pub destination: state::DeckPath,
    pub card_id: ids::CardId,
}

impl PlayerTask for MoveCardTask {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        game.move_card(self.source, self.destination, self.card_id)?;
        Ok(TaskProgressResult::TaskComplete)
    }
}

// #[derive(Debug, Clone)]
// pub struct RollForAbilityTask {
//     pub player_index: usize,
//     pub card_id: ids::CardId,
// }

// impl PlayerTask for RollForAbilityTask {
//     fn make_progress(
//         &mut self,
//         context: &mut game_context::GameBookKeeping,
//         game: &mut state::Game,
//     ) -> SlayResult<TaskProgressResult> {
//     }
// }

#[derive(Debug, Clone)]
pub struct UseAbilityTask {
    pub deck_path: state::DeckPath,
    pub card_id: ids::CardId,
}

impl PlayerTask for UseAbilityTask {
    fn make_progress(
        &mut self,
        context: &mut game_context::GameBookKeeping,
        game: &mut state::Game,
    ) -> SlayResult<TaskProgressResult> {
        // do the ability!!
        // Implement it!
        println!("We got here!");
        Ok(TaskProgressResult::TaskComplete)
    }
}
