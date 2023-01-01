use yew::classes;
use yew::prelude::*;

use crate::common::perspective::PlayerCompletionPerspective;
use crate::frontend::icons::Continue;
use crate::frontend::icons::Done;

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
