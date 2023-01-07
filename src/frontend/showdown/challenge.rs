use yew::classes;
use yew::prelude::*;

use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::dice::Dice;
use crate::frontend::icons::Timer;
use crate::frontend::showdown::common::CompletionsView;
use crate::frontend::showdown::common::RollChoices;
use crate::frontend::showdown::common::RollHistory;
use crate::frontend::showdown::common::RollTotal;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::showdown::challenge::ChallengePerspective;
use crate::slay::showdown::challenge::ChallengeRollPerspective;
use crate::slay::showdown::common::ChallengeReason;

#[function_component(ChallengeDescription)]
pub fn view_challenge_description(props: &ChallengeModalProps) -> Html {
	let text = match &props.challenge.reason {
		ChallengeReason::PlaceHeroCard(spec) => html! {
			<>
				<div class={classes!("row")}>
				{
					format!(
						"{} challenged {}'s choice to place ",
						props.challenge.challenger.roller_name,
						props.challenge.initiator.roller_name,
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
					{
						format!("in his party.")
					}
				</div>
			</>
		},
		ChallengeReason::PlaceItem(spec) => html! {
			<>
				<div class={classes!("row")}>
				{
					format!(
						"{} challenged {}'s choice to place the item ",
						props.challenge.challenger.roller_name,
						props.challenge.initiator.roller_name,
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
				</div>
			</>
		},
		ChallengeReason::CastMagic(spec) => html! {
			<>
				<div class={classes!("row")}>
				{
					format!(
						"{} challenged {}'s choice to play magic card",
						props.challenge.challenger.roller_name,
						props.challenge.initiator.roller_name,
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
				</div>
			</>
		},
	};
	html! {
		{text}
	}
}

#[derive(Properties, PartialEq)]
pub struct ChallengeRollProps {
	pub challenge: ChallengePerspective,
	pub callbacks: GameCallbacks,
	pub roll: ChallengeRollPerspective,
}

#[function_component(ChallengeRollView)]
pub fn view_challenge_roll(props: &ChallengeRollProps) -> Html {
	let _open = use_state(|| false);
	html! {
		<div
			class={classes!("column")}
		>
			<label>{format!("{}'s roll", props.roll.roller_name)}</label>
			<Dice roll={props.roll.initial.to_owned()}/>
			<br/>
			<RollHistory history={props.roll.history.to_owned()}/>
			<br/>
			<RollTotal success={false} amount={props.roll.roll_total}/>
			<br/>
			<RollChoices choices={props.roll.choices.to_owned()} callbacks={props.callbacks.to_owned()}/>
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct ChallengeModalProps {
	pub challenge: ChallengePerspective,
	pub callbacks: GameCallbacks,
}

#[function_component(ChallengeModalView)]
pub fn view_challenge_modal(props: &ChallengeModalProps) -> Html {
	let _open = use_state(|| false);
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<ChallengeDescription
					challenge={props.challenge.to_owned()}
					callbacks={props.callbacks.to_owned()}
				/>
				<br/>
				<Timer timeline={props.challenge.timeline.to_owned()}/>
				<br/>
				<div class={classes!("row")}>
					<ChallengeRollView
						challenge={props.challenge.to_owned()}
						callbacks={props.callbacks.to_owned()}
						roll={props.challenge.initiator.to_owned()}
					/>
					<ChallengeRollView
						challenge={props.challenge.to_owned()}
						callbacks={props.callbacks.to_owned()}
						roll={props.challenge.challenger.to_owned()}
					/>
				</div>
				<br/>
				<CompletionsView completions={props.challenge.completions.to_owned()}/>
				<br/>
				<RollChoices
					choices={props.challenge.choices.to_owned()}
					callbacks={props.callbacks.to_owned()}
				/>
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
