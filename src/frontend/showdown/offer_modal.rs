use yew::classes;
use yew::prelude::*;

use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::icons::DoNot;
use crate::frontend::icons::Timer;
use crate::frontend::showdown::common::CompletionsView;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::showdown::common::ChallengeReason;
use crate::slay::showdown::offer::OfferChallengesPerspective;

#[derive(Properties, PartialEq)]
pub struct OfferModalProps {
	pub offer: OfferChallengesPerspective,
	pub callbacks: GameCallbacks,
}

#[function_component(OfferDescriptionView)]
pub fn view_offer_context(props: &OfferModalProps) -> Html {

	let description = match &props.offer.reason {
    ChallengeReason::PlaceHeroCard(spec) => html! {
			<div class={classes!("row")}>
				{props.offer.initiator.to_owned()}
				{"is placing"}
				<CardSpecView
					spec={spec.to_owned()}
					view_card={props.callbacks.view_card.to_owned()}
					choice_state={ChoiceState::default()}
					extra_specs={ExtraSpecProps::default()}
				/>
				{"In their party."}
			</div>
		},
    ChallengeReason::PlaceItem(spec) => html! {
			<div class={classes!("row")}>
			{props.offer.initiator.to_owned()}
			{"wants to place the item"}
			<CardSpecView
				spec={spec.to_owned()}
				view_card={props.callbacks.view_card.to_owned()}
				choice_state={ChoiceState::default()}
				extra_specs={ExtraSpecProps::default()}
			/>
			</div>
		},
    ChallengeReason::CastMagic(spec) => html! {
			<div class={classes!("row")}>
			{"wants to cast the magic card"}
			<CardSpecView
				spec={spec.to_owned()}
				view_card={props.callbacks.view_card.to_owned()}
				choice_state={ChoiceState::default()}
				extra_specs={ExtraSpecProps::default()}
			/>
			</div>
		},
	};

	html! {
		<div>
			{ description }
			<br/>
			{"Would you like to challenge?"}
		</div>
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
		// html! {
		// 	<div
		// 		onclick={choose_this}
		// 	>
		// 		{choice.label.to_owned()}
		// 	</div>
		// }
		match choice.display_type {
			ChoiceDisplayType::Challenge(_) => html! {
				<div
					onclick={choose_this}
				>
					<img
						src={"imgs/icons/challenge.png"}
						alt={"Go back"}
						width={100}
					/>
					// {choice.label.to_owned()}
				</div>
			},
			ChoiceDisplayType::SetCompletion(_) => html! {
				<div
					onclick={choose_this}
				>
					<DoNot/>
				</div>
			},
			_ => todo!(),
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
