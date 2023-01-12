use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::deck::DeckView;
use crate::slay::state::player::PlayerPerspective;

#[derive(Properties, PartialEq)]
struct ActionPointsProps {
	total: u32,
	remaining: u32,
}

#[function_component(ActionPoints)]
fn view_action_points(props: &ActionPointsProps) -> Html {
	let points = (0..props.total).map(|point_index| {
		html! {
				<div class={classes!("action-point", if point_index < props.remaining {
						"unused"
				} else {
						"used"
				})}/>
		}
	});
	html! {
			<div
					class={classes!("action-points")}
					title={format!("{} of {} action points remain.",
							props.remaining, props.total)}
			>
					{ for points }
			</div>
	}
}

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
	pub player: PlayerPerspective,
	pub common: Rc<CommonProps>,
}

#[function_component(PlayerView)]
pub fn view_player(props: &PlayerProps) -> Html {
	let mut is_choice = None;
	if !props.player.choices(props.common.get_choices()).is_empty() {
		is_choice = Some("is-choice".to_owned());
	}

	let decks = props.player.decks.iter().map(|deck| {
		html! {
				<DeckView
						deck={deck.to_owned()}
						common={props.common.to_owned()}
				/>
		}
	});
	let clazz = if props.player.is_active(&props.common.perspective) {
		"active-player"
	} else {
		"inactive-player"
	};

	let title = format!(
		"{} {} {}",
		props.player.name(&props.common.statics).to_owned(),
		if props.player.is_active(props.common.perspective.as_ref()) {
			"(active player)"
		} else {
			""
		},
		if props.player.is_me(props.common.statics.as_ref()) {
			"(that's you!)"
		} else {
			""
		},
	);
	// TODO: Show the leader card!!
	html! {
			<div class={classes!(clazz, is_choice)}>
					<div class={classes!("player-status")}>
							{title}
							<ActionPoints
									total={props.player.total_action_points}
									remaining={props.player.remaining_action_points}
							/>
							<div/> // For buffs
							<div/> // For hero types in party
					</div>
					<div class={classes!("decks")}>
							{for decks}
					</div>
			</div>
	}
}
