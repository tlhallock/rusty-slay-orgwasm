use crate::slay::deadlines::Timeline;

use gloo_timers::callback::Interval;

use yew::classes;
use yew::prelude::*;

#[function_component(DoNot)]
pub fn do_not() -> Html {
	html! {
		<svg width="100" height="100">
			<circle cx="50" cy="50" r="40" stroke="red" stroke-width="10"/>
			<line x1="25" y1="25" x2="75" y2="75" stroke="red" stroke-width="10" />
		</svg>
	}
}

#[function_component(Done)]
pub fn done() -> Html {
	html! {
		<svg width="100" height="100">
			<line x1="0" y1="66" x2="33" y2="95" stroke="green" stroke-width="15" />
			<line x1="28" y1="95" x2="95" y2="5" stroke="green" stroke-width="15" />
		</svg>
	}
}

#[function_component(Continue)]
pub fn thinking() -> Html {
	html! {
		<svg width="100" height="100">
			<circle cx="25" cy="50" r="8" fill="yellow" class={classes!("thinking-1")}/>
			<circle cx="50" cy="50" r="8" fill="yellow" class={classes!("thinking-2")}/>
			<circle cx="75" cy="50" r="8" fill="yellow" class={classes!("thinking-3")}/>
		</svg>
	}
}

#[derive(Properties, PartialEq)]
pub struct TimerProps {
	pub timeline: Timeline,
}

#[function_component(Timer)]
pub fn timer(props: &TimerProps) -> Html {
	let current_completion = use_state(|| props.timeline.completion());

	let callback = {
		let timeline = props.timeline.clone();
		move || current_completion.set(timeline.completion())
	};
	let timeout = Interval::new(20, callback);
	timeout.forget();

	// gloo timer
	let completion_option = props.timeline.completion();
	if let Some(completion) = completion_option {
		let mut width = (800f64 * completion.percent_complete) as i32;
		if width < 0 {
			width = 0;
		}
		if width > 800 {
			width = 800;
		}
		let width = format!("{}", width);
		html! {
			<div>
				<label>{format!("{:.2}s", completion.seconds_remaining)}</label>
				<br/>
				<svg width="800" height="10">
					<rect x="0" y="0" width="800" height="10" fill="yellow"/>
					<rect x="0" y="0" width={width} height="10" fill="blue"/>
				</svg>
			</div>
		}
	} else {
		html! {
			<div/>
		}
	}
}
