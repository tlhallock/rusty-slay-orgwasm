use yew::classes;
use yew::prelude::*;

use crate::frontend::deck::DeckView;
use crate::slay::state::player::PlayerPerspective;
use crate::frontend::app::ChoiceState;
use crate::frontend::card_modal::CardModalInfo;

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
	pub view_card: Callback<Option<CardModalInfo>, ()>,
	pub choice_state: ChoiceState,
}

#[function_component(PlayerView)]
pub fn view_player(props: &PlayerProps) -> Html {
	let decks = props.player.decks.iter().map(|deck| {
		html! {
				<DeckView
						deck={deck.to_owned()}
						view_card={props.view_card.to_owned()}
						choice_state={props.choice_state.to_owned()}
				/>
		}
	});
	let clazz = if props.player.active {
		"active-player"
	} else {
		"inactive-player"
	};

	let title = format!(
		"{} {} {}",
		props.player.name.to_owned(),
		if props.player.active {
			"(active player)"
		} else {
			""
		},
		if props.player.me { "(that's you!)" } else { "" },
	);
	html! {
			<div class={classes!(clazz)}>
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
