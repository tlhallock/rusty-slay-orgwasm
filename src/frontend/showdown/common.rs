use yew::classes;
use yew::prelude::*;

use crate::common::perspective::CardSpecPerspective;
use crate::common::perspective::ChoicePerspective;
use crate::common::perspective::ModificationPerspective;
use crate::common::perspective::PlayerCompletionPerspective;
use crate::common::perspective::RollModificationChoiceType;
use crate::frontend::app::ChoiceState;
use crate::frontend::app::GameCallbacks;
use crate::frontend::icons::Continue;
use crate::frontend::icons::Done;

use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::showdown::completion::RollCompletion;

use crate::frontend::icons::DoNot;

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
						RollCompletion::Thinking => html! { <Continue/> },
						RollCompletion::DoneUntilModification => html! { <Done/> },
						RollCompletion::AllDone => html! { <DoNot/> },
					}
				}
			</div>
		}
	});
	html! {
		<div class={classes!{"completions"}}>
			{ for completions }
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct RollHistoryProps {
	pub history: Vec<ModificationPerspective>,
}

#[function_component(RollHistory)]
pub fn view_roll_history(props: &RollHistoryProps) -> Html {
	let completions = props.history.iter().map(|m| {
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
	let choices = props
		.choices
		.iter()
		.filter(|choice| choice.roll_modification_choice.is_some())
		.map(|choice| {
			let choose_this = {
				let choose = props.callbacks.choose.clone();
				let choice_id = choice.choice_id;
				move |_| choose.iter().for_each(|c| c.emit(choice_id))
			};
			match &choice
				.roll_modification_choice
				.as_ref()
				.unwrap()
				.choice_type
			{
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
				RollModificationChoiceType::Nothing(persist) => match persist {
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
