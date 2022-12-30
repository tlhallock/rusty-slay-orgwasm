use yew::classes;
use yew::prelude::*;

use log;

use crate::common::perspective::CardSpecPerspective;
use crate::common::perspective::ChoiceAssociationType;
use crate::common::perspective::StackPerspective;

use super::app::ChoiceState;
use super::card_modal::CardModalInfo;

#[derive(Properties, PartialEq, Default)]
pub struct ExtraSpecProps {
	pub is_choice_representation: bool,
	pub is_highlighted_choice: bool,
	pub is_default_choice: bool,
	pub has_been_played_this_turn: bool,
}

impl ExtraSpecProps {
	fn get_css_class(&self) -> Option<&'static str> {
		if self.is_choice_representation {
			return Some("is_choice");
		}
		return Some("is-part-of-choice");
	}
}

#[derive(Properties, PartialEq)]
pub struct SpecProps {
	pub spec: CardSpecPerspective,
	pub view_card: Callback<Option<CardModalInfo>, ()>,
	pub choice_state: ChoiceState,
	pub extra_specs: ExtraSpecProps,
}

#[function_component(CardSpecView)]
pub fn view_spec(props: &SpecProps) -> Html {
	let view_this_card = {
		let view_card = props.view_card.clone();
		let modal = props.spec.to_card_modal();
		move |_| view_card.emit(Some(modal.clone()))
	};

	let c = props.extra_specs.get_css_class();

	html! {
			<div
					class={classes!("card", c)}
					onclick={view_this_card}
			>
					{ props.spec.label.to_owned() }
			</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct StackProps {
	pub stack: StackPerspective,
	pub view_card: Callback<Option<CardModalInfo>, ()>,
	pub choice_state: ChoiceState,
}

impl StackProps {
	pub fn create_extra_props(&self) -> ExtraSpecProps {
		ExtraSpecProps {
			is_choice_representation: self
				.stack
				.top
				.choice_associations
				.iter()
				.any(|a| a.association_type == ChoiceAssociationType::Representer),
			is_highlighted_choice: self.choice_state.highlighted_choice.iter().any(|id| {
				self
					.stack
					.top
					.choice_associations
					.iter()
					.any(|a| a.choice_id == *id)
			}),
			is_default_choice: self
				.stack
				.top
				.choice_associations
				.iter()
				.any(|a| a.is_default),
			has_been_played_this_turn: self.stack.top.played_this_turn,
		}
	}
}

#[function_component(StackView)]
pub fn view_stack(props: &StackProps) -> Html {
	html! {
			<CardSpecView
					spec={props.stack.top.spec.to_owned()}
					view_card={props.view_card.to_owned()}
					choice_state={props.choice_state.to_owned()}
					extra_specs={props.create_extra_props()}
			/>
	}
}
