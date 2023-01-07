use yew::classes;
use yew::prelude::*;


use crate::slay::choices::ChoiceAssociation;
use crate::slay::state::stack::CardSpecPerspective;

use super::app::GameCallbacks;

#[derive(Properties, PartialEq)]
pub struct CardModalProps {
	pub info: CardModalInfo,
	pub callbacks: GameCallbacks,
}

#[function_component(CardModalView)]
pub fn view_card_details(props: &CardModalProps) -> Html {
	let clear_card = {
		let view_card = props.callbacks.view_card.clone();
		move |_| view_card.emit(None)
	};
	let choices = props.info.represented.iter().map(|choice| {
		let choose_this = {
			let choose = props.callbacks.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};
		html! {
			<div>
				<button
					class={classes!("choice-button")}
					onclick={choose_this}
				>
					{choice.label.to_owned()}
				</button>
			</div>
		}
	});
	html! {
			<div class={classes!("card-modal")} onclick={clear_card}>
					<div class={classes!("modal-content")}>
							<h1>
									{props.info.label.to_owned()}
							</h1>
							<br/>
							<label>
									{props.info.description.to_owned()}
							</label>
							<br/>
							{for choices}
							<br/>
							<img
									src={format!("imgs/{}", props.info.image_path)}
									alt={props.info.description.to_owned()}
									width={500}
							/>
					</div>
					// <button>{"ok"}</button>
			</div>
	}
}

#[derive(Clone, PartialEq)]
pub struct CardModalInfo {
	pub image_path: String,
	pub description: String,
	pub label: String,
	pub represented: Vec<ChoiceAssociation>,
}

impl CardSpecPerspective {
	pub fn to_card_modal(&self, represented: Vec<ChoiceAssociation>) -> CardModalInfo {
		CardModalInfo {
			image_path: self.image_path.to_owned(),
			description: self.description.to_owned(),
			label: self.label.to_owned(),
			represented,
		}
	}
}
