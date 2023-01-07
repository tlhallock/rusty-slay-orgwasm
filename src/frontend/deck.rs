use yew::classes;
use yew::prelude::*;

use crate::frontend::stack::StackView;
use crate::slay::state::deck::DeckPerspective;

use super::app::ChoiceState;
use super::card_modal::CardModalInfo;

#[derive(Properties, PartialEq)]
pub struct DeckProps {
	pub deck: DeckPerspective,
	pub view_card: Callback<Option<CardModalInfo>, ()>,
	pub choice_state: ChoiceState,
}

#[function_component(DeckView)]
pub fn view_deck(props: &DeckProps) -> Html {
	let deck_value = if let Some(stacks) = &props.deck.stacks {
		html! {
				<div class={classes!("cards")}>
						{
								for stacks.iter().map(
										|s| html! {
												<StackView
														stack={s.to_owned()}
														view_card={props.view_card.to_owned()}
														choice_state={props.choice_state.to_owned()}
												/>
										}
								)
						}
				</div>
		}
	} else {
		html! {
				<div class={classes!("many-cards")}>
						// {props.deck.count} {" cards"}

						{
								for (0..props.deck.count).map(
										|index| html! {
												<div class={classes!("many-card-wrapper")}>
														<div class={classes!("hidden-card")}>
														{ if index == props.deck.count - 1 && props.deck.count > 7 {
																format!("{} cards", props.deck.count)
														} else {
																"".to_string()
														}}
														</div>
												</div>
										}
								)
						}
				</div>
		}
	};
	html! {
			<div class={classes!("deck-holder")}>
					{props.deck.label.to_owned()}
					{": "}
					{deck_value}
			</div>
	}
}
