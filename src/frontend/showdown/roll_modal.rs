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
use crate::slay::showdown::consequences::Comparison;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::roll_state::RollPerspective;
use crate::slay::showdown::roll_state::RollReason;

#[derive(Properties, PartialEq)]
pub struct SimplerRollModalProps {
	pub roll: RollPerspective,
}

#[derive(Properties, PartialEq)]
pub struct RollModalProps {
	pub roll: RollPerspective,
	pub callbacks: GameCallbacks,
}

fn format_condition(condition: &Condition) -> String {
	match condition.cmp {
		Comparison::LE => format!("<= {}", condition.threshold),
		Comparison::GE => format!(">= {}", condition.threshold),
	}
}

// fn format_consequences(consequences: &Vec<TaskSpec>) -> String {
// 	return "".to_owned();
// 	// consequences
// 	// 	.iter()
// 	// 	.map(|spec| match spec {
// 	// 		TaskSpec::Sacrifice(num) => format!("Sacrifice {} heros.", num),
// 	// 		TaskSpec::Discard(num) => format!("Discard {} cards.", num),
// 	// 		// These are not the bad things...
// 	// 		TaskSpec::ReceiveModifier(_) => unreachable!(),
// 	// 		TaskSpec::Draw(_) => unreachable!(),
// 	// 	})
// 	// 	.collect()
// }

#[function_component(RollDescription)]
pub fn view_roll_context(props: &RollModalProps) -> Html {
	let text = match &props.roll.reason {
		RollReason::UseHeroAbility(spec) => html! {
			<>
				<div class={classes!("row")}>
				{
					format!(
						"{} is rolling for ",
						props.roll.roller_name,
					)
				}
					<CardSpecView
						spec={spec.to_owned()}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
					{
						format!(
							"'s ability'",
						)
					}
				</div>
				<br/>
				{
					"todo"
					// format!(
					// 	// TODO: add probability XD
					// 	"If the roll is {}, the player be able to {}.",
					// 	format_condition(&spec.hero_ability.as_ref().unwrap().success_condition),
					// 	spec.description,
					// )
				}
				<br/>
			</>
		},
		RollReason::AttackMonster(spec) => html! {
			<>
				<div class={classes!("row")}>
					{
						format!(
							"{} is attacking ",
							props.roll.roller_name,
						)
					}
					<CardSpecView
						spec={spec.to_owned()}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
				</div>
				<br/>
				// {
				// 	format!(
				// 		// TODO: add probability XD
				// 		"If the roll is {}, the player be have: {}.",
				// 		format_condition(&spec.monster.as_ref().unwrap().victory.condition),
				// 		spec.description,
				// 	)
				// }
				// <br/>
				// {
				// 	format!(
				// 		"If the roll is {}, the player be have to: .",
				// 		format_condition(&spec.monster.as_ref().unwrap().loss.condition),
				// 		// format_consequences(&spec.monster.as_ref().unwrap().loss.tasks),
				// 	)
				// }
				// <br/>
				// {
				// 	format!(
				// 		"This player currently has slain {} monsters.",
				// 		"<implement me!>",
				// 	)
				// }
				<br/>
			</>
		},
	};
	html! {
		<label>
			// {"The instructions go here."}
			<br/>
			{text}
		</label>
	}
}

#[function_component(RollTimer)]
fn view_roll_timer(props: &SimplerRollModalProps) -> Html {
	html! {
		<div>
			// <label>{format!("This roll times out at {:?}", props.roll.deadline)}</label>
			<br/>
			<Timer timeline={props.roll.timeline.to_owned()}/>
		</div>
	}
}

#[function_component(RollModalView)]
pub fn view_roll_modal(props: &RollModalProps) -> Html {
	let _open = use_state(|| false);
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<RollDescription roll={props.roll.to_owned()} callbacks={props.callbacks.to_owned()}/>
				<br/>
				<RollTimer roll={props.roll.to_owned()}/>
				<br/>
				<Dice roll={props.roll.initial.to_owned()}/>
				<br/>
				<RollHistory history={props.roll.history.to_owned()}/>
				<br/>
				<RollTotal success={props.roll.success} amount={props.roll.roll_total}/>
				<br/>
				<RollChoices choices={props.roll.choices.to_owned()} callbacks={props.callbacks.to_owned()}/>
				<br/>
				<CompletionsView completions={props.roll.completions.to_owned()}/>
				<br/>
				<div>
				</div>
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
