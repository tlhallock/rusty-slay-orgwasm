use yew::prelude::*;

use log;

#[derive(Properties, PartialEq)]
pub struct MessagesProps {
	pub messages: Vec<String>,
}

#[function_component(MessagesView)]
pub fn view_messages(props: &MessagesProps) -> Html {
	html! {
			<>
					<span>{"Messages:"}</span>
					{
							for props.messages.iter().map(
									|m| html! {
											<span>{m}</span>
									}
							)
					}
			</>
	}
}
