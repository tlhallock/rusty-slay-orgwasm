use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use log;

use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::card_modal::CardModalInfo;
use crate::frontend::card_modal::CardModalView;
use crate::frontend::choices::ChoicesView;
use crate::frontend::deck::DeckView;
use crate::frontend::player::PlayerView;
use crate::frontend::showdown::challenge::ChallengeModalView;
use crate::frontend::showdown::offer_modal::OfferChallengesView;
use crate::frontend::showdown::roll_modal::RollModalView;
use crate::slay::ids;
use crate::slay::state::game::GamePerspective;

#[derive(Properties, PartialEq)]
pub struct GamePerspectiveProps {
	pub game: Rc<GamePerspective>,
	pub choose: Option<Callback<ids::ChoiceId, ()>>,
}

#[function_component(GamePerspectiveView)]
pub fn view_game(props: &GamePerspectiveProps) -> Html {
	let viewed_card = use_state(|| None::<CardModalInfo>);
	let view_card = {
		let viewed_card = viewed_card.clone();
		Callback::from(move |m| viewed_card.set(m))
	};
	let choice_state = use_state(ChoiceState::default);
	let set_selected_choice = {
		let choice_state = choice_state.clone();
		Callback::from(move |highlighted_choice| {
			log::info!("Trying to set highlight");
			choice_state.set(ChoiceState { highlighted_choice })
		})
	};

	let rotated = props.game.rotated_players();
	let players = rotated.iter().map(|player| {
		html! {
				<PlayerView
						player={(*player).to_owned()}
						view_card={view_card.to_owned()}
						choice_state={(*choice_state).to_owned()}
				/>
		}
	});
	let decks = props.game.decks.iter().map(|deck| {
		html! {
				<DeckView
						deck={deck.to_owned()}
						view_card={view_card.to_owned()}
						choice_state={(*choice_state).to_owned()}
				/>
		}
	});
	let card_view = viewed_card.as_ref().map(|m| {
		html! {
				<CardModalView info={m.to_owned()} callbacks={GameCallbacks {
					choose: props.choose.to_owned(),
					view_card: view_card.to_owned(),
				}} />
		}
	});
	let choices_instructions = props.game.choices.as_ref().map(|c| {
		html! {
				<ChoicesView
						choices={c.to_owned()}
						callbacks={GameCallbacks {
							choose: props.choose.to_owned(),
							view_card: view_card.to_owned(),
						}}
						set_selected_choice={set_selected_choice.to_owned()}
				/>
		}
	});
	let roll = props
		.game
		.roll
		.as_ref()
		// vec![].iter()
		.map(|r| {
			html! {
					<RollModalView roll={r.to_owned()} callbacks={
						GameCallbacks {
							choose: props.choose.to_owned(),
							view_card: view_card.to_owned(),
						}
					}/>
			}
		});
	let offer = props
		.game
		.offer
		.as_ref()
		// vec![].iter()
		.map(|o| {
			html! {
				<OfferChallengesView
					offer={o.to_owned()}
					callbacks={
						GameCallbacks {
							choose: props.choose.to_owned(),
							view_card: view_card.to_owned(),
						}
					}
				/>
			}
		});

	let challenge = props.game.challenge.as_ref().map(|c| {
		html! {
			<ChallengeModalView
				challenge={c.to_owned()}
				callbacks={
					GameCallbacks {
						choose: props.choose.to_owned(),
						view_card: view_card.to_owned(),
					}
				}
			/>
		}
	});
	html! {
			<div>
					{for choices_instructions}
					{for card_view}
					{for roll}
					{for offer}
					{for challenge}
					<div class={classes!("global-decks")}>
							{for decks}
					</div>
					<div class={classes!("players")}>
							{for players}
					</div>
			</div>
	}
}

/*



						<svg width="100" height="100">
						<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
						<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

						<circle cx="50" cy="50" r="5" fill="#000000"/>
					</svg>


					<svg width="100" height="100">
	<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
	<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

	<circle cx="30" cy="30" r="5" fill="#000000"/>
	<circle cx="70" cy="70" r="5" fill="#000000"/>
</svg>


				<svg width="100" height="100">
				<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
				<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

				<circle cx="30" cy="30" r="5" fill="#000000"/>
				<circle cx="70" cy="70" r="5" fill="#000000"/>
				<circle cx="50" cy="50" r="5" fill="#000000"/>
			</svg>




				<svg width="100" height="100">
				<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
				<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

				<circle cx="30" cy="30" r="5" fill="#000000"/>
				<circle cx="70" cy="30" r="5" fill="#000000"/>
				<circle cx="30" cy="70" r="5" fill="#000000"/>
				<circle cx="70" cy="70" r="5" fill="#000000"/>
			</svg>

			<svg width="100" height="100">
	<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
	<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

	<circle cx="30" cy="30" r="5" fill="#000000"/>
	<circle cx="70" cy="30" r="5" fill="#000000"/>
	<circle cx="30" cy="70" r="5" fill="#000000"/>
	<circle cx="70" cy="70" r="5" fill="#000000"/>
	<circle cx="50" cy="50" r="5" fill="#000000"/>
</svg>







<svg width="100" height="100">
	<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
	<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

	<circle cx="30" cy="30" r="5" fill="#000000"/>
	<circle cx="30" cy="50" r="5" fill="#000000"/>
	<circle cx="30" cy="70" r="5" fill="#000000"/>
	<circle cx="70" cy="30" r="5" fill="#000000"/>
	<circle cx="70" cy="50" r="5" fill="#000000"/>
	<circle cx="70" cy="70" r="5" fill="#000000"/>
</svg>




				</div>

				*/
