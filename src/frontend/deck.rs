use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::stack::StackView;
use crate::slay::state::deck::DeckPerspective;

use super::app::CommonProps;

#[derive(Properties, PartialEq)]
pub struct DeckProps {
	pub deck: DeckPerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(DeckView)]
pub fn view_deck(props: &DeckProps) -> Html {
	let mut is_choice = None;
	if !props.deck.choices(props.common.get_choices()).is_empty() {
		is_choice = Some("is-choice".to_owned());
	}
	let deck_value = if let Some(stacks) = &props.deck.stacks {
		html! {
				<div class={classes!("cards", is_choice)}>
						{
								for stacks.iter().map(
										|s| html! {
												<StackView
														stack={s.to_owned()}
														common={props.common.to_owned()}
												/>
										}
								)
						}
				</div>
		}
	} else {
		html! {
				<div class={classes!("many-cards", is_choice)}>
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
					{props.deck.path.get_label()}
					{": "}
					{deck_value}
			</div>
	}
}
