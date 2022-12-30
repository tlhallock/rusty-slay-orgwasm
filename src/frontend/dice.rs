use yew::classes;
use yew::prelude::*;



use crate::slay::showdown::common::Roll;

#[function_component(Die1)]
fn view_die_1() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<circle cx="50" cy="50" r="5" fill="#000000"/>
			</svg>
	}
}

#[function_component(Die2)]
fn view_die_2() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<circle cx="30" cy="30" r="5" fill="#000000"/>
					<circle cx="70" cy="70" r="5" fill="#000000"/>
			</svg>
	}
}

#[function_component(Die3)]
fn view_die_3() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<circle cx="30" cy="30" r="5" fill="#000000"/>
					<circle cx="70" cy="70" r="5" fill="#000000"/>
					<circle cx="50" cy="50" r="5" fill="#000000"/>
			</svg>
	}
}

#[function_component(Die4)]
fn view_die_4() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

					<circle cx="30" cy="30" r="5" fill="#000000"/>
					<circle cx="70" cy="30" r="5" fill="#000000"/>
					<circle cx="30" cy="70" r="5" fill="#000000"/>
					<circle cx="70" cy="70" r="5" fill="#000000"/>
			</svg>
	}
}

#[function_component(Die5)]
fn view_die_5() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>

					<circle cx="30" cy="30" r="5" fill="#000000"/>
					<circle cx="70" cy="30" r="5" fill="#000000"/>
					<circle cx="30" cy="70" r="5" fill="#000000"/>
					<circle cx="70" cy="70" r="5" fill="#000000"/>
					<circle cx="50" cy="50" r="5" fill="#000000"/>
			</svg>
	}
}

#[function_component(Die6)]
fn view_die_6() -> Html {
	html! {
			<svg width="100" height="100">
					<rect x="0" y="0" width="100" height="100" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<rect x="10" y="10" width="80" height="80" rx="10" ry="10" fill="#ffffff" stroke="#000000" stroke-width="2"/>
					<circle cx="30" cy="30" r="5" fill="#000000"/>
					<circle cx="30" cy="50" r="5" fill="#000000"/>
					<circle cx="30" cy="70" r="5" fill="#000000"/>
					<circle cx="70" cy="30" r="5" fill="#000000"/>
					<circle cx="70" cy="50" r="5" fill="#000000"/>
					<circle cx="70" cy="70" r="5" fill="#000000"/>
			</svg>
	}
}

#[derive(Properties, PartialEq)]
struct DieProps {
	pub top: u32,
}

#[function_component(Die)]
fn view_die(props: &DieProps) -> Html {
	match props.top {
		1 => html! {<div class={classes!("die")}><Die1/></div>},
		2 => html! {<div class={classes!("die")}><Die2/></div>},
		3 => html! {<div class={classes!("die")}><Die3/></div>},
		4 => html! {<div class={classes!("die")}><Die4/></div>},
		5 => html! {<div class={classes!("die")}><Die5/></div>},
		6 => html! {<div class={classes!("die")}><Die6/></div>},
		_ => panic!(),
	}
}

#[derive(Properties, PartialEq)]
pub struct DiceProps {
	pub roll: Roll,
}

#[function_component(Dice)]
pub fn view_dice(props: &DiceProps) -> Html {
	html! {
			<div class={classes!("dice")}>
					<Die top={props.roll.die1}/>
					<Die top={props.roll.die2}/>
			</div>
	}
}
