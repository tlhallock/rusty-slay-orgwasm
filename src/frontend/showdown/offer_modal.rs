use yew::classes;
use yew::prelude::*;

use crate::frontend::icons::Timer;

use crate::frontend::app::GameCallbacks;

use crate::frontend::showdown::common::CompletionsView;
use crate::slay::showdown::offer::OfferChallengesPerspective;

#[derive(Properties, PartialEq)]
pub struct OfferModalProps {
	pub offer: OfferChallengesPerspective,
	pub callbacks: GameCallbacks,
}

#[function_component(OfferDescriptionView)]
pub fn view_offer_context(_props: &OfferModalProps) -> Html {
	let text = html! {<div></div>};
	html! {
		<label>{"The instructions go here."}<br/>{text}</label>
	}
}

#[function_component(OfferChallengesCoices)]
fn view_offer_choices(props: &OfferModalProps) -> Html {
	let choices = props.offer.choices.iter().map(|choice| {
		let choose_this = {
			let choose = props.callbacks.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};

		html! {
			<div
				onclick={choose_this}
			>
				{choice.label.to_owned()}
			</div>
		}
	});
	html! {
		<div class={classes!("roll-choices")}>
			{for choices}
		</div>
	}
}

#[function_component(OfferChallengesView)]
pub fn view_roll_modal(props: &OfferModalProps) -> Html {
	let _open = use_state(|| false);
	// let clear_card = {
	//     let view_card = props.view_card.clone();
	//     move |_| view_card.emit(None)
	// };
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<OfferDescriptionView offer={props.offer.to_owned()} callbacks={props.callbacks.to_owned()}/>
				<br/>
				<Timer timeline={props.offer.timeline.to_owned()}/>
				<br/>
				<OfferChallengesCoices offer={props.offer.to_owned()} callbacks={props.callbacks.to_owned()}/>
				<br/>
				<CompletionsView completions={props.offer.completions.to_owned()}/>
				<br/>
				<div>
					<img
						src={"imgs/icons/back.png"}
						alt={"Go back"}
						width={50}
					/>
				</div>
			</div>
		</div>
	}
}
