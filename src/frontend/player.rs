use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::deck::DeckView;
use crate::frontend::stack::CardSpecView;
use crate::frontend::stack::ExtraSpecProps;
use crate::slay::state::player::PlayerPerspective;
use crate::slay::state::player::RepresentedHeroType;

#[derive(Properties, PartialEq)]
struct RepresentedHeroTypeProps {
	represented_hero_types: Vec<RepresentedHeroType>,
}

#[function_component(HeroTypesView)]
fn view_hero_types(props: &RepresentedHeroTypeProps) -> Html {
	let represented = props
		.represented_hero_types
		.iter()
		.filter(|hero_type| hero_type.represented)
		.map(|hero_type| {
			html! {
				<img
					width={40}
					src={hero_type.hero_type.icon().to_owned()}
					alt={hero_type.hero_type.label().to_owned()}
				/>
			}
		});
	html! {
			<div
					class={classes!("action-points")}
			>
					{ for represented }
			</div>
	}
}

#[derive(Properties, PartialEq)]
struct ActionPointsProps {
	total: u32,
	remaining: u32,
}

#[function_component(ActionPoints)]
fn view_action_points(props: &ActionPointsProps) -> Html {
	let points = (0..props.total).map(|point_index| {
		if point_index < props.remaining {
			html! {
				<div class={classes!("action-point", "unused")}/>
				// <img
				// 	width={20}
				// 	src="imgs/icons/action_point.png"
				// 	alt="action point"
				// />
			}
		} else {
			html! {
				<div class={classes!("action-point", "used")}/>
			}
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
							<HeroTypesView
								represented_hero_types={
									props.player.represented_hero_types.to_vec()
								}
							/> // For hero types in party
					</div>
					<div class={classes!("decks")}>
						<CardSpecView
								spec={props.player.leader.spec.to_owned()}
								common={props.common.to_owned()}
								extra_specs={
									ExtraSpecProps {
										represented_choices: Vec::new(), // TODO
										is_default_choice: false,
										has_been_played_this_turn: props.player.leader.played_this_turn,
										disable_view: false,
									}
								}
						/>
						{for decks}
					</div>
			</div>
	}
}
