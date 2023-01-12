use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::icons::Continue;
use crate::frontend::icons::DoNot;
use crate::frontend::icons::Done;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::choices::ChoiceDisplayType;
use crate::slay::choices::ChoicePerspective;
use crate::slay::ids;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::showdown::common::ModificationOrigin;
use crate::slay::showdown::common::RollModification;
use crate::slay::showdown::common::RollModificationChoiceType;
use crate::slay::showdown::completion::Completion;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::modifier::ModifierKinds;

#[derive(Properties, PartialEq)]
pub struct CompletionsProps {
	pub completions: Vec<Completion>,
	pub common: Rc<CommonProps>,
}

#[function_component(CompletionsView)]
pub fn view_roll_completions(props: &CompletionsProps) -> Html {
	let completions = props
		.completions
		.iter()
		.enumerate()
		.map(|(player_index, c)| {
			html! {
				<div class={classes!{"completion"}}>
					{ props.common.statics.player_name(player_index).to_owned() }
					{
						match c {
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
pub struct ModificationAmountProps {
	pub amount: i32,
}
#[function_component(RollModificationAmount)]
pub fn view_roll_modification_amount(props: &ModificationAmountProps) -> Html {
	if props.amount < 0 {
		html! {
			<div class={classes!("roll-choice-minus")}>{ format!("{}", props.amount) } </div>
		}
	} else {
		html! {
			<div class={classes!("roll-choice-plus")}>{ format!("+{}", props.amount) } </div>
		}
	}
}

#[derive(Properties, PartialEq)]
pub struct PlayerModificationViewProps {
	pub common: Rc<CommonProps>,
	pub player_index: ids::PlayerIndex,
	pub kinds: ModifierKinds,
	pub amount: i32,
}
#[function_component(PlayerModificationView)]
pub fn view_player_roll_modification(props: &PlayerModificationViewProps) -> Html {
	html! {
		<div class={classes!("row")}>
			<div class={classes!("username")}>
				{props.common.statics.player_name(props.player_index)}
			</div>
			 <CardSpecView
				 spec={SlayCardSpec::ModifierCard(props.kinds)}
				common={props.common.to_owned()}
				extra_specs={ExtraSpecProps::default()}
			/>
			 <RollModificationAmount amount={props.amount}/>
		 </div>
	}
}
#[derive(Properties, PartialEq)]
pub struct BuffModificationViewProps {
	pub common: Rc<CommonProps>,
	pub origin: ModifierOrigin,
	pub amount: i32,
}
#[function_component(BuffModificationView)]
pub fn view_buff_roll_modification(props: &BuffModificationViewProps) -> Html {
	html! {
		<div>
			{ "Implement buff modifications" }
			<RollModificationAmount amount={props.amount}/>
		</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct RollModificationProps {
	pub common: Rc<CommonProps>,
	pub modification: RollModification,
}
#[function_component(RollModificationView)]
pub fn view_roll_modification(props: &RollModificationProps) -> Html {
	match props.modification.origin {
		ModificationOrigin::FromPlayer(player_index, kinds) => html! {
			<PlayerModificationView
				common={props.common.to_owned()}
				player_index={player_index}
				kinds={kinds.to_owned()}
				amount={props.modification.amount}
			/>
		},
		ModificationOrigin::FromBuff(origin) => html! {
			<BuffModificationView
				common={props.common.to_owned()}
				origin={origin.to_owned()}
				amount={props.modification.amount}
			/>
		},
	}
}

#[derive(Properties, PartialEq)]
pub struct RollHistoryProps {
	pub common: Rc<CommonProps>,
	pub history: Vec<RollModification>,
}

#[function_component(RollHistory)]
pub fn view_roll_history(props: &RollHistoryProps) -> Html {
	let history = props.history.iter().map(|m| {
		html! {
			<RollModificationView
				common={props.common.to_owned()}
				modification={m.to_owned()}
			/>
		}
	});
	html! {
		<div
			class={classes!("column")}
		>
			{"Modifications:"}
			{ for history }
		</div>
	}
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
	pub common: Rc<CommonProps>,
}

// impl IntoPropValue<SlayCardSpec> for ModifierKinds {

// }

#[function_component(RollChoices)]
pub fn view_roll_choices(props: &RollChoicesProps) -> Html {
	let choices = props.choices.iter().map(|choice| {
		let choose_this = {
			let choose = props.common.choose.clone();
			let choice_id = choice.choice_id;
			move |_| choose.iter().for_each(|c| c.emit(choice_id))
		};
		match &choice.display.display_type {
			ChoiceDisplayType::Modify(modi) => match modi {
				RollModificationChoiceType::AddToRoll(kind, amount, _) => html! {
					<div
						onclick={choose_this}
						title={format!("Modify this card by {}", amount)}
					>
						<CardSpecView
							spec={SlayCardSpec::ModifierCard(*kind)}
							common={props.common.to_owned()}
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
				RollModificationChoiceType::RemoveFromRoll(kind, amount, _) => html! {
					<div
						onclick={choose_this}
						title={format!("Modify this card by {}", amount)}
					>
						<CardSpecView
							spec={SlayCardSpec::ModifierCard(*kind)}
							common={props.common.to_owned()}
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
