use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::icons::DoNot;
use crate::frontend::icons::Timer;
use crate::frontend::showdown::common::CompletionsView;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::showdown::offer::OfferChallengesPerspective;
use crate::slay::showdown::roll::ChallengeReason;

#[derive(Properties, PartialEq)]
pub struct OfferModalProps {
	pub offer: OfferChallengesPerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(OfferDescriptionView)]
pub fn view_offer_context(props: &OfferModalProps) -> Html {
	// TODO: Could use a username...
	let description = match &props.offer.reason {
		ChallengeReason::PlaceHeroCard(spec) => html! {
			<div class={classes!("row")}>
				{props.common.statics.player_name(props.offer.player_index).to_owned()}
				{"is placing"}
				<CardSpecView
					spec={spec.to_owned()}
					common={props.common.to_owned()}
					extra_specs={ExtraSpecProps::default()}
				/>
				{"In their party."}
			</div>
		},
		ChallengeReason::PlaceItem(spec) => html! {
			<div class={classes!("row")}>
			{props.common.statics.player_name(props.offer.player_index).to_owned()}
			{"wants to place the item"}
			<CardSpecView
				spec={spec.to_owned()}
				common={props.common.to_owned()}
				extra_specs={ExtraSpecProps::default()}
			/>
			</div>
		},
		ChallengeReason::CastMagic(spec) => html! {
			<div class={classes!("row")}>
				{props.common.statics.player_name(props.offer.player_index).to_owned()}
				{"wants to cast the magic card"}
				<CardSpecView
					spec={spec.to_owned()}
					common={props.common.to_owned()}
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
	let choices = props.offer.choices(props.common.get_choices());
	let choices = choices.iter().map(|choice| {
		let choose_this = {
			let choose = props.common.choose.clone();
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
		match choice.display {
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
	// Does this need to be one higher?
	// TODO: DRY?
	let is_open = use_state(|| true);
	let close = {
		let open_handle = is_open.clone();
		Callback::from(move |_| open_handle.set(false))
	};
	let open = {
		let open_handle = is_open.clone();
		Callback::from(move |_| open_handle.set(true))
	};
	if !*is_open {
		return html! {
			<button onclick={open}>
				{ "Back to challenge offers" }
			</button>
		};
	}
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<OfferDescriptionView
					offer={props.offer.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<Timer timeline={props.offer.completion_tracker.timeline.to_owned()}/>
				<br/>
				<OfferChallengesCoices
					offer={props.offer.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<CompletionsView
					completions={props.offer.completion_tracker.completions.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<div onclick={close}>
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
