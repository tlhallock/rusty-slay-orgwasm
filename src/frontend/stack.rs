use yew::classes;
use yew::prelude::*;

use crate::slay::choices::ChoiceAssociation;
use crate::slay::choices::ChoiceAssociationType;
use crate::slay::state::stack::CardSpecPerspective;
use crate::slay::state::stack::StackPerspective;

use super::app::ChoiceState;
use super::card_modal::CardModalInfo;

#[derive(Properties, PartialEq, Default)]
pub struct ExtraSpecProps {
	pub represented_choices: Vec<ChoiceAssociation>,
	pub is_highlighted_choice: bool,
	pub is_default_choice: bool,
	pub has_been_played_this_turn: bool,
	pub disable_view: bool,
}

impl ExtraSpecProps {
	fn get_css_class(&self) -> Option<&'static str> {
		if !self.represented_choices.is_empty() {
			return Some("is-choice");
		}
		Some("is-part-of-choice")
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
		let modal = props
			.spec
			.to_card_modal(props.extra_specs.represented_choices.to_owned());
		move |_| view_card.emit(Some(modal.clone()))
	};

	let c = props.extra_specs.get_css_class();
	if props.extra_specs.disable_view {
		html! {
			<div
					class={classes!("card", c)}
			>
					{ props.spec.label.to_owned() }
			</div>
		}
	} else {
		html! {
			<div
				class={classes!("card", c)}
				onclick={view_this_card}
			>
				{ props.spec.label.to_owned() }
			</div>
		}
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
		// for association in self.stack.top.choice_associations.iter() {
		// 	log::info!("Association type: {:?} {}",
		// 		association,
		// 		self
		// 		.stack
		// 		.top
		// 		.choice_associations
		// 		.iter()
		// 		.any(|a| a.association_type == ChoiceAssociationType::Representer),
		// 	);
		// }

		ExtraSpecProps {
			represented_choices: self
				.stack
				.top
				.choice_associations
				.iter()
				.filter(|a| a.association_type == ChoiceAssociationType::Representer)
				.map(|a| a.to_owned())
				.collect(),
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
			disable_view: false,
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
