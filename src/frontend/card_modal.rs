use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::slay::choices::ChoicePerspective;
use crate::slay::specs::cards::SlayCardSpec;

#[derive(Clone, PartialEq)]
pub struct CardModalInfo {
	pub spec: SlayCardSpec,
	// TODO: Make this a method as well...
	// hopefully this is only 1
	pub represents: Vec<ChoicePerspective>,
}

#[derive(Properties, PartialEq)]
pub struct CardModalProps {
	pub info: CardModalInfo,
	pub common: Rc<CommonProps>,
}

#[function_component(CardModalView)]
pub fn view_card_details(props: &CardModalProps) -> Html {
	let clear_card = {
		let view_card = props.common.view_card.clone();
		move |_| view_card.emit(None)
	};
	let choices = props.info.represents.iter().map(|choice| {
		let choose_this = {
			let choose = props.common.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};
		html! {
			<div>
				<button
					class={classes!("choice-button")}
					onclick={choose_this}
				>
					{choice.choice.label()}
				</button>
			</div>
		}
	});
	html! {
			<div class={classes!("card-modal")} onclick={clear_card}>
					<div class={classes!("modal-content")}>
							<h1>
									{props.info.spec.get_card_spec_creation().label}
							</h1>
							<br/>
							<label>
									{props.info.spec.get_card_spec_creation().description}
							</label>
							<br/>
							{for choices}
							<br/>
							<img
									src={props.info.spec.get_card_spec_creation().get_image_path()}
									alt={props.info.spec.get_card_spec_creation().description}
									width={500}
							/>
					</div>
					// <button>{"ok"}</button>
			</div>
	}
}
