use crate::slay::choices::ChoiceDisplay;
use crate::slay::choices::Choices;
use crate::slay::choices::TasksChoice;
use crate::slay::deadlines;
use crate::slay::game_context::GameBookKeeping;
use crate::slay::ids;
use crate::slay::state::deck::DeckPath;
use crate::slay::state::game::Game;
use crate::slay::state::stack::Card;
use crate::slay::tasks::tasks::move_card::MoveCardTask;

pub enum SearchDiscardFilters {
	IsHero,
}

impl SearchDiscardFilters {
	fn filter(&self, card: &Card) -> bool {
		match self {
			SearchDiscardFilters::IsHero => card.is_hero(),
		}
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
		instructions: "Choose a hero card to add to your hand.".to_owned(),
		default_choice: None,
		options,
		timeline: deadlines::get_refactor_me_deadline(), // This one should probably be longer...
	})
}
