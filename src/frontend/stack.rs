use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::card_modal::CardModalInfo;
use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::ChoicesPerspective;
use crate::slay::specs::cards::card_type::SlayCardSpec;
use crate::slay::state::stack::StackPerspective;

#[derive(Properties, PartialEq, Default)]
pub struct ExtraSpecProps {
	pub represented_choices: Vec<ChoicePerspective>,
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
	pub spec: SlayCardSpec,
	pub common: Rc<CommonProps>,
	pub extra_specs: ExtraSpecProps,
}

#[function_component(CardSpecView)]
pub fn view_spec(props: &SpecProps) -> Html {
	let view_this_card = {
		let view_card = props.common.view_card.clone();
		let modal = CardModalInfo {
			spec: props.spec,
			represents: props.extra_specs.represented_choices.to_owned(),
		};
		move |_| view_card.emit(Some(modal.clone()))
	};

	let c = props.extra_specs.get_css_class();
	if props.extra_specs.disable_view {
		html! {
			<div
					class={classes!("card", c)}
			>
					{ props.spec.label() }
			</div>
		}
	} else {
		html! {
			<div
				class={classes!("card", c)}
				onclick={view_this_card}
			>
				{ props.spec.label() }
				<br/>
				{ for
					if let Some(hero_type) = props.spec.hero_type() {
						Some(html! {
							<img
								width={30}
								src={hero_type.icon().to_owned()}
								alt={hero_type.label()}
							/>
						})
						// Some(hero_type.label().to_owned())
					} else {
						None
					}
				}
			</div>
		}
	}
}

#[derive(Properties, PartialEq)]
pub struct StackProps {
	pub stack: StackPerspective,
	pub common: Rc<CommonProps>,
}

impl StackProps {
	pub fn create_extra_props(&self, choices: &Option<ChoicesPerspective>) -> ExtraSpecProps {
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
		let choices = if let Some(choices) = choices {
			choices.represents_card(self.stack.top.id) // TODO;
		} else {
			Vec::new()
		};
		let is_default_choice = choices.iter().any(|c| c.is_default);
		ExtraSpecProps {
			represented_choices: choices,
			is_default_choice,
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
					common={props.common.to_owned()}
					extra_specs={props.create_extra_props(props.common.get_choices())}
			/>
	}
}
