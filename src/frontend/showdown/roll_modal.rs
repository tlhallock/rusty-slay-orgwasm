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
use crate::slay::showdown::roll_state::RollPerspective;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::specs::cards::card_type::SlayCardSpec;

#[derive(Properties, PartialEq)]
pub struct SimplerRollModalProps {
	pub roll: RollPerspective,
}

#[derive(Properties, PartialEq)]
pub struct RollModalProps {
	pub roll: RollPerspective,
	pub common: Rc<CommonProps>,
}

// fn format_condition(condition: &Condition) -> String {
// 	match condition.cmp {
// 		Comparison::LE => format!("<= {}", condition.threshold),
// 		Comparison::GE => format!(">= {}", condition.threshold),
// 	}
// }

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
						props.roll.roller_name(&props.common.statics).to_owned()
					)
				}
					<CardSpecView
						spec={SlayCardSpec::HeroCard(*spec)}
						common={props.common.to_owned()}
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
							props.roll.roller_name(&props.common.statics).to_owned()
						)
					}
					<CardSpecView
						spec={SlayCardSpec::MonsterCard(*spec)}
						common={props.common.to_owned()}
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
		_ => unreachable!(),
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
			<Timer timeline={props.roll.completion_tracker.timeline.to_owned()}/>
		</div>
	}
}

#[function_component(RollModalView)]
pub fn view_roll_modal(props: &RollModalProps) -> Html {
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
				{ "Back to roll modifications" }
			</button>
		};
	}
	html! {
		<div class={classes!("modal")}>
			<div class={classes!("modal-content")}>
				<RollDescription
					roll={props.roll.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				<RollTimer roll={props.roll.to_owned()}/>
				<br/>
				<Dice roll={props.roll.initial.to_owned()}/>
				<br/>
				<RollHistory
					history={props.roll.history.to_owned()}
					common={props.common.to_owned()}
				/>
				<br/>
				// TODO: add a not failed...
				<RollTotal
					success={props.roll.won()}
					amount={props.roll.calculate_roll_total()}
				/>
				<br/>
				<RollChoices
					choices={props.roll.choices(props.common.get_choices())}
					common={props.common.to_owned()}
				/>
				<br/>
				<CompletionsView
					completions={props.roll.completion_tracker.completions.to_vec()}
					common={props.common.to_owned()}
				/>
				<br/>
				<div>
				</div>
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
