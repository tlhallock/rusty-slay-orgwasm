use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::Choices;
use crate::slay::choices::ChoicesType;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::errors::SlayResult;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::player_tasks::TaskProgressResult;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

#[derive(Clone, Debug, Copy)]
pub enum SearchDiscardFilters {
	IsHero,
	IsModifier,
	IsItem,
	IsMagic,
}

impl SearchDiscardFilters {
	fn filter(&self, card: &Card) -> bool {
		match self {
			SearchDiscardFilters::IsHero => card.is_hero(),
			SearchDiscardFilters::IsModifier => card.is_modifier(),
			SearchDiscardFilters::IsItem => card.is_item(),
			SearchDiscardFilters::IsMagic => card.is_magic(),
		}
	}
}

#[derive(Clone, Debug)]
pub struct SearchDiscard {
	filters: SearchDiscardFilters,
}

impl SearchDiscard {
	pub fn for_modifiers() -> Box<dyn PlayerTask> {
		Box::new(Self {
			filters: SearchDiscardFilters::IsModifier,
		})
	}
	pub fn for_hero() -> Box<dyn PlayerTask> {
		Box::new(Self {
			filters: SearchDiscardFilters::IsHero,
		})
	}
	pub fn for_item() -> Box<dyn PlayerTask> {
		Box::new(Self {
			filters: SearchDiscardFilters::IsItem,
		})
	}
	pub fn for_magic() -> Box<dyn PlayerTask> {
		Box::new(Self {
			filters: SearchDiscardFilters::IsMagic,
		})
	}
}

impl PlayerTask for SearchDiscard {
	fn make_progress(
		&mut self,
		context: &mut GameBookKeeping,
		game: &mut Game,
		player_index: ids::PlayerIndex,
	) -> SlayResult<TaskProgressResult> {
		game.players[player_index].choices =
			create_search_discard_choices(context, game, player_index, self.filters);
		Ok(TaskProgressResult::TaskComplete)
	}

	fn label(&self) -> String {
		format!("Search the discard pile for for {:?}", self.filters)
	}
}

pub fn create_search_discard_choices(
	context: &mut GameBookKeeping,
	game: &mut Game,
	player_index: ids::PlayerIndex,
	filter: SearchDiscardFilters,
) -> Option<Choices> {
	let options = game
		.deck(DeckPath::Discard)
		.tops()
		.filter(|card| filter.filter(card))
		.map(|card| {
			TasksChoice::new(
				context.id_generator.generate(),
				ChoiceDisplay {
					display_type: card.as_choice(),
					label: card.get_spec().label,
				},
				vec![Box::new(MoveCardTask {
					source: DeckPath::Discard,
					destination: DeckPath::Hand(player_index),
					card_id: card.id,
				})],
			)
		})
		.collect::<Vec<_>>();
	if options.is_empty() {
		return None;
	}
	Some(Choices {
		choices_type: ChoicesType::SearchDiscard(filter),
		instructions: "Choose a hero card to add to your hand.".to_owned(),
		default_choice: None,
		options,
		timeline: deadlines::get_refactor_me_deadline(), // This one should probably be longer...
	})
}
