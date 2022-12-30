use yew::classes;
use yew::prelude::*;

use log;

use crate::common::perspective::ChoicePerspective;
use crate::common::perspective::ChoicesPerspective;
use crate::slay::ids;

#[derive(Properties, PartialEq)]
pub struct ChoiceProps {
	choice: ChoicePerspective,
	choose: Callback<ids::ChoiceId, ()>,
	set_selected_choice: Callback<Option<ids::ChoiceId>, ()>,
}

#[function_component(ChoiceView)]
pub fn view_choices(props: &ChoiceProps) -> Html {
	let choose_this_choice = {
		let choose = props.choose.clone();
		let choice_id = props.choice.choice_id;
		move |_| choose.emit(choice_id)
	};
	let select_this_choice = {
		let set_selected_choice = props.set_selected_choice.clone();
		let choice_id = props.choice.choice_id;
		move |_| set_selected_choice.emit(Some(choice_id))
	};
	let remove_any_selected_choice = {
		let set_selected_choice = props.set_selected_choice.clone();
		let choice_id = props.choice.choice_id;
		move |_| set_selected_choice.emit(None)
	};

	html! {
			<button
					onclick={choose_this_choice}
					onmouseenter={select_this_choice}
					onmouseleave={remove_any_selected_choice}
			>
					{props.choice.label.to_owned()}
			</button>
	}
}

#[derive(Properties, PartialEq)]
pub struct ChoicesInstructionsProps {
	pub choices: ChoicesPerspective,
	pub choose: Callback<ids::ChoiceId, ()>,
	pub set_selected_choice: Callback<Option<ids::ChoiceId>, ()>,
}

#[function_component(ChoicesView)]
pub fn view_choices(props: &ChoicesInstructionsProps) -> Html {
	html! {
			<>
					<span>{props.choices.instructions.to_owned()}</span>
					{
							for props.choices.actions.iter().map(
									|c| html! {
											<ChoiceView
													choice={c.to_owned()}
													choose={props.choose.to_owned()}
													set_selected_choice={props.set_selected_choice.to_owned()}
											/>
									}
							)
					}
			</>
	}
}
