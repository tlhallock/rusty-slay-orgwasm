use yew::classes;
use yew::prelude::*;

use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::icons::Continue;
use crate::frontend::icons::DoNot;
use crate::frontend::icons::Done;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::ChoicePerspective;
use crate::slay::showdown::common::ModificationPerspective;
use crate::slay::showdown::common::RollModificationChoiceType;
use crate::slay::showdown::completion::Completion;
use crate::slay::showdown::completion::PlayerCompletionPerspective;

#[derive(Properties, PartialEq)]
pub struct CompletionsProps {
	pub completions: Vec<PlayerCompletionPerspective>,
}

#[function_component(CompletionsView)]
pub fn view_roll_completions(props: &CompletionsProps) -> Html {
	let completions = props.completions.iter().map(|c| {
		html! {
			<div class={classes!{"completion"}}>
				{ c.player_name.to_owned() }
				{
					match c.completion {
						Completion::Thinking => html! { <Continue/> },
						Completion::DoneUntilModification => html! { <Done/> },
						Completion::AllDone => html! { <DoNot/> },
					}
				}
			</div>
		}
	});
	html! {
		<div class={classes!{"completions"}}>
			// {"Completions:"}
			{ for completions }
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct RollHistoryProps {
	pub history: Vec<ModificationPerspective>,
	pub callbacks: GameCallbacks,
}

#[function_component(RollHistory)]
pub fn view_roll_history(props: &RollHistoryProps) -> Html {
	let history = props.history.iter().map(|m| {
		html! {
			 <div class={classes!("row")}>
				  <div class={classes!("username")}>
					  {m.modifier_name.to_owned()}
				 </div>
				 <CardSpecView
					 spec={m.modifying_card_spec.to_owned()}
					 view_card={props.callbacks.view_card.to_owned()}
					 choice_state={ChoiceState::default()}
					 extra_specs={ExtraSpecProps::default()}
				 />
				 {
					if m.modification_amount < 0 {
						html! {
							<div class={classes!("roll-choice-minus")}>{ format!("{}", m.modification_amount) } </div>
						}
					} else {
						html! {
							<div class={classes!("roll-choice-plus")}>{ format!("+{}", m.modification_amount) } </div>
						}
					}
				 }
			 </div>
		 }
	});
	html! { <div class={classes!("column")}> {"Modifications:"}{ for history } </div> }
}

#[derive(Properties, PartialEq)]
pub struct RollTotalProps {
	pub amount: i32,
	pub success: bool,
}

#[function_component(RollTotal)]
pub fn view_roll_result(props: &RollTotalProps) -> Html {
	html! {
		<div class={classes!(if props.success {"roll-total-success" } else {"roll-total-failure"})}>
			{props.amount}
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct RollChoicesProps {
	pub choices: Vec<ChoicePerspective>,
	pub callbacks: GameCallbacks,
}

#[function_component(RollChoices)]
pub fn view_roll_choices(props: &RollChoicesProps) -> Html {
	let choices = props.choices.iter().map(|choice| {
		let choose_this = {
			let choose = props.callbacks.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};
		match &choice.display_type {
			ChoiceDisplayType::Modify(modi) => match modi {
				RollModificationChoiceType::AddToRoll(spec, amount, _) => html! {
					<div
						onclick={choose_this}
						title={format!("Modify this card by {}", amount)}
					>
						<CardSpecView
							spec={spec.to_owned()}
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
				RollModificationChoiceType::RemoveFromRoll(spec, amount, _) => html! {
					<div
						onclick={choose_this}
						title={format!("Modify this card by {}", amount)}
					>
						<CardSpecView
							spec={spec.to_owned()}
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
			},
			ChoiceDisplayType::SetCompletion(comp) => match comp {
				Completion::Thinking => todo!(),
				Completion::AllDone => html! {
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
				Completion::DoneUntilModification => html! {
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
			},
			_ => unreachable!(),
		}
	});
	html! {
		<div class={classes!("roll-choices", if props.choices.is_empty() { None } else { Some("is-choice") })}>
			// {"Options:"}
			{for choices}
		</div>
	}
}
