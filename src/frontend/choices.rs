use std::rc::Rc;

use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::slay::choices::ChoicePerspective;
use crate::slay::choices::ChoicesPerspective;

#[derive(Properties, PartialEq)]
pub struct ChoiceProps {
	choice: ChoicePerspective,
	common: Rc<CommonProps>,
}

#[function_component(ChoiceView)]
pub fn view_choices(props: &ChoiceProps) -> Html {
	let choose_this_choice = {
		let choose = props.common.choose.clone().unwrap();
		let choice_id = props.choice.choice_id;
		move |_| choose.emit(choice_id)
	};
	let select_this_choice = {
		let set_highlighted_choice = props.common.set_highlighted_choice.clone();
		let choice_id = props.choice.choice_id;
		move |_| set_highlighted_choice.emit(Some(choice_id))
	};
	let remove_any_selected_choice = {
		let set_highlighted_choice = props.common.set_highlighted_choice.clone();
		let _choice_id = props.choice.choice_id;
		move |_| set_highlighted_choice.emit(None)
	};

	html! {
		<button
			class={classes!("choice-button")}
			onclick={choose_this_choice}
			onmouseenter={select_this_choice}
			onmouseleave={remove_any_selected_choice}
		>
			{props.choice.display.label.to_owned()}
		</button>
	}
}

#[derive(Properties, PartialEq)]
pub struct ChoicesInstructionsProps {
	pub choices: ChoicesPerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(ChoicesView)]
pub fn view_choices(props: &ChoicesInstructionsProps) -> Html {
	html! {
			<>
					<span>{props.choices.instructions.to_owned()}</span>
					{
							for props.choices.options.iter().map(
									|c| html! {
											<ChoiceView
													choice={c.to_owned()}
													common={props.common.to_owned()}
											/>
									}
							)
					}
			</>
	}
}
