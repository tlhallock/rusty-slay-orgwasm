use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::dice::Dice;
use crate::frontend::icons::Timer;
use crate::frontend::showdown::common::CompletionsView;
use crate::frontend::showdown::common::RollChoices;
use crate::frontend::showdown::common::RollHistory;
use crate::frontend::showdown::common::RollTotal;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::showdown::challenge::ChallengePerspective;
use crate::slay::showdown::roll::ChallengeReason;

#[function_component(ChallengeDescription)]
pub fn view_challenge_description(props: &ChallengeModalProps) -> Html {
	let text = match &props.challenge.reason {
		ChallengeReason::PlaceHeroCard(spec) => html! {
			<>
				<div class={classes!("row")}>
				{
					format!(
						"{} challenged {}'s choice to place ",
						props.challenge.challenger.roller_name(&props.common.statics),
						props.challenge.initiator.roller_name(&props.common.statics),
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						common={props.common.to_owned()}
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
						props.challenge.challenger.roller_name(&props.common.statics),
						props.challenge.initiator.roller_name(&props.common.statics),
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						common={props.common.to_owned()}
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
						props.challenge.challenger.roller_name(&props.common.statics),
						props.challenge.initiator.roller_name(&props.common.statics),
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						common={props.common.to_owned()}
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

// #[derive(Properties, PartialEq)]
// pub struct ChallengeRollProps {
// 	pub challenge: ChallengePerspective,
// 	pub callbacks: GameCallbacks,
// 	pub roll: ChallengeRollPerspective,
// 	pub success: bool,
// }

// #[function_component(ChallengeRollView)]
// pub fn view_challenge_roll(props: &ChallengeRollProps) -> Html {
// 	let _open = use_state(|| false);
// 	html! {
// 		<div
// 			class={classes!("column")}
// 		>
// 			<label>{format!("{}'s roll", props.roll.roller_name)}</label>
// 			<Dice roll={props.roll.initial.to_owned()}/>
// 			<br/>
// 			<RollHistory history={props.roll.history.to_owned()} callbacks={props.callbacks.to_owned()}/>
// 			<br/>
// 			<RollTotal success={props.success} amount={props.roll.roll_total}/>
// 			<br/>
// 			<RollChoices choices={props.roll.choices.to_owned()} callbacks={props.callbacks.to_owned()}/>
// 		</div>
// 	}
// }

#[derive(Properties, PartialEq)]
pub struct ChallengeRollProps {
	pub challenge: ChallengePerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(ChallengeRollsView)]
pub fn view_challenge_roll(props: &ChallengeRollProps) -> Html {
	let _open = use_state(|| false);
	html! {
		<div
			class={classes!("column")}
		>
			<div class={classes!("row")}>
				<div class={classes!("column")}>
					<label>
						{
							format!(
								"{}'s roll",
								props.challenge.initiator.roller_name(&props.common.statics)
							)
						}
					</label>
					<Dice roll={props.challenge.initiator.initial.to_owned()}/>
				</div>
				<div class={classes!("column")}>
					<label>
						{
							format!(
								"{}'s roll",
								props.challenge.challenger.roller_name(&props.common.statics)
							)
						}
					</label>
					<Dice roll={props.challenge.challenger.initial.to_owned()}/>
				</div>
			</div>
			<div class={classes!("row")}>
					<RollHistory
						history={props.challenge.initiator.history.to_owned()}
						common={props.common.to_owned()}
					/>
					<RollHistory
						history={props.challenge.challenger.history.to_owned()}
						common={props.common.to_owned()}
					/>
			</div>
			<div class={classes!("row")}>
					<RollTotal
						success={!props.challenge.is_challenger_victories()}
						amount={props.challenge.initiator.calculate_roll_total()}
					/>
					<RollTotal
						success={props.challenge.is_challenger_victories()}
						amount={props.challenge.challenger.calculate_roll_total()}
					/>
			</div>
			<div class={classes!("row")}>
				<div class={classes!("column")}>
					<RollChoices
						choices={props.challenge.initiator.choices(props.common.get_choices())}
						common={props.common.to_owned()}
					/>
				</div>
				<div class={classes!("column")}>
					<RollChoices
						choices={props.challenge.challenger.choices(props.common.get_choices())}
						common={props.common.to_owned()}
					/>
				</div>
			</div>
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct ChallengeModalProps {
	pub challenge: ChallengePerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(ChallengeModalView)]
pub fn view_challenge_modal(props: &ChallengeModalProps) -> Html {
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
				{ "Back to challenge modifications" }
			</button>
		};
	}
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<ChallengeDescription
					challenge={props.challenge.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<Timer timeline={props.challenge.completion_tracker.timeline.to_owned()}/>
				<br/>
				<ChallengeRollsView
					challenge={props.challenge.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<CompletionsView
					completions={props.challenge.completion_tracker.completions.to_vec()}
					common={props.common.to_owned()}
				/>
				<br/>
				<RollChoices
					choices={props.challenge.choices(props.common.get_choices())}
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
