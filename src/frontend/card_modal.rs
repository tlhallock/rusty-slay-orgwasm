use yew::classes;
use yew::prelude::*;

use crate::common::perspective::CardSpecPerspective;

#[derive(Properties, PartialEq)]
pub struct CardModalProps {
	pub info: CardModalInfo,
	pub view_card: Callback<Option<CardModalInfo>, ()>,
}

#[function_component(CardModalView)]
pub fn view_card_details(props: &CardModalProps) -> Html {
	let clear_card = {
		let view_card = props.view_card.clone();
		move |_| view_card.emit(None)
	};
	html! {
			<div class={classes!("modal")} onclick={clear_card}>
					<div class={classes!("modal-content")}>
							<h1>
									{props.info.label.to_owned()}
							</h1>
							<br/>
							<label>
									{props.info.description.to_owned()}
							</label>
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
}

impl CardSpecPerspective {
	pub fn to_card_modal(&self) -> CardModalInfo {
		CardModalInfo {
			image_path: self.image_path.to_owned(),
			description: self.description.to_owned(),
			label: self.label.to_owned(),
		}
	}
}
