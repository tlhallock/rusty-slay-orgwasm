use yew::classes;
use yew::prelude::*;

use crate::common::perspective::CardSpecPerspective;
use crate::common::perspective::RollModificationChoiceType;
use crate::common::perspective::RollPerspective;
use crate::frontend::dice::Dice;
use crate::frontend::icons::Done;
use crate::frontend::icons::Timer;
use crate::frontend::showdown::common::CompletionsView;
use crate::frontend::stack::CardSpecView;

use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::icons::DoNot;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::showdown::completion::RollCompletion;
use crate::slay::showdown::consequences::Comparison;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::tasks::TaskSpec;

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

fn format_consequences(consequences: &Vec<TaskSpec>) -> String {
	consequences
		.iter()
		.map(|spec| match spec {
			TaskSpec::Sacrifice(num) => format!("Sacrifice {} heros.", num),
			TaskSpec::Discard(num) => format!("Discard {} cards.", num),
			// These are not the bad things...
			TaskSpec::ReceiveModifier(_) => unreachable!(),
			TaskSpec::Draw(_) => unreachable!(),
		})
		.collect()
}

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
						spec={CardSpecPerspective::new(spec)}
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
					format!(
						// TODO: add probability XD
						"If the roll is {}, the player be able to {}.",
						format_condition(&spec.hero_ability.as_ref().unwrap().success_condition),
						spec.description,
					)
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
						spec={CardSpecPerspective::new(spec)}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={ExtraSpecProps::default()}
					/>
				</div>
				<br/>
				{
					format!(
						// TODO: add probability XD
						"If the roll is {}, the player be have: {}.",
						format_condition(&spec.monster.as_ref().unwrap().victory.condition),
						spec.description,
					)
				}
				<br/>
				{
					format!(
						"If the roll is {}, the player be have to: {}.",
						format_condition(&spec.monster.as_ref().unwrap().loss.condition),
						format_consequences(&spec.monster.as_ref().unwrap().loss.tasks),
					)
				}
				<br/>
				{
					format!(
						"This player currently has slain {} monsters.",
						"<implement me!>",
					)
				}
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

#[function_component(RollInitial)]
pub fn view_initial_roll(props: &SimplerRollModalProps) -> Html {
	html! {
			<Dice roll={props.roll.initial.to_owned()}/>
	}
}

// #[function_component(UseAbilityConsequence)]
// pub fn view_use_ability_consequence(props: &SimplerRollModalProps) -> Html {
// 	html! {
// 			<Dice roll={props.roll.initial.to_owned()}/>
// 	}
// }

// #[function_component(MonsterConsequence)]
// pub fn view_attack_monster_consequence(props: &SimplerRollModalProps) -> Html {
// 	html! {
// 			<Dice roll={props.roll.initial.to_owned()}/>
// 	}
// }

#[function_component(RollTotal)]
pub fn view_roll_result(props: &SimplerRollModalProps) -> Html {
	// let consequence = match props.roll.reason {
	//   RollReason::UseHeroAbility(_) => html! {<MonsterConsequence roll={props.roll.to_owned()}/>},
	//   RollReason::AttackMonster(_) => html! {<MonsterConsequence roll={props.roll.to_owned()}/>},
	// };
	html! {
		<div class={classes!(if props.roll.success {"roll-total-success" } else {"roll-total-failure"})}>
			{props.roll.roll_total}
		</div>
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

#[function_component(RollChoices)]
fn view_roll_choices(props: &RollModalProps) -> Html {
	let choices = props.roll.choices.iter().map(|choice| {
		let choose_this = {
			let choose = props.callbacks.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};
		match &choice.choice_type {
			RollModificationChoiceType::AddToRoll(spec, amount) => html! {
				<div
					onclick={choose_this}
					title={format!("Modify this card by {}", amount)}
				>
					<CardSpecView
						spec={CardSpecPerspective::new(&spec)}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={
							ExtraSpecProps {
								disable_view: true,
								..Default::default()
							}
						}
					/>
					<div class={classes!("roll-choice-plus")}>{ format!("+{}", amount) } </div>
				</div>
			},
			RollModificationChoiceType::RemoveFromRoll(spec, amount) => html! {
				<div
					onclick={choose_this}
					title={format!("Modify this card by {}", amount)}
				>
					<CardSpecView
						spec={CardSpecPerspective::new(&spec)}
						view_card={props.callbacks.view_card.to_owned()}
						choice_state={ChoiceState::default()}
						extra_specs={
							ExtraSpecProps {
								disable_view: true,
								..Default::default()
							}}
					/>
					<div class={classes!("roll-choice-minus")}>{ format!("{}", amount) } </div>
				</div>
			},
			RollModificationChoiceType::Nothing(persist) => match *persist {
				RollCompletion::AllDone => html! {
					<div
						onclick={choose_this}
						title={"Do not modify this roll."}
					>
						<div  class={classes!("roll-choice-plus")}>
							<DoNot/>
							// <img
							// 	src={"imgs/icons/no.jpeg"}
							// 	alt={"Do not modify this roll"}
							// 	width={70}
							// />
						</div>
					</div>
				},
				RollCompletion::DoneUntilModification => html! {
					<div
						onclick={choose_this}
						title={"Do not modify this roll, unless someone else does."}
					>
						<div  class={classes!("roll-choice-plus")}>
							// <img
							// 	src={"imgs/icons/no.jpeg"}
							// 	alt={"Do not modify this roll, unless someone else does."}
							// 	width={50}
							// />
							<Done/>
						</div>
					</div>
				},
				_ => {
					unreachable!();
				}
			},
		}
	});
	html! {
		<div class={classes!("roll-choices")}>
			{for choices}
		</div>
	}
}

#[function_component(RollHistory)]
fn view_roll_history(props: &SimplerRollModalProps) -> Html {
	let completions = props.roll.history.iter().map(|m| {
		html! {
			 <label>
				 { format!("Player {} used {} to modify by {}.",
					 m.modifyer_name,
					 "<implement this>",
					 m.modification_amount,
		) }
			 </label>
		 }
	});
	html! { <> { for completions } </> }
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
				<RollInitial roll={props.roll.to_owned()}/>
				<br/>
				<RollHistory roll={props.roll.to_owned()}/>
				<br/>
				<RollTotal roll={props.roll.to_owned()}/>
				<br/>
				<RollChoices roll={props.roll.to_owned()} callbacks={props.callbacks.to_owned()}/>
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
