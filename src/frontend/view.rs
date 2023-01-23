use std::rc::Rc;

use yew::prelude::*;

use crate::frontend::app::AppState;
use crate::frontend::game::GamePerspectiveView;
use crate::slay::specs::cards::SlayCardSpec;
use crate::slay::specs::hero::HeroAbilityType;

// View the notifications
// View the hero leader...
// Make players clickable?

#[function_component(App)]
fn app() -> Html {
	let current_game: UseStateHandle<AppState> = use_state(AppState::new);
	let _player_index = use_state(|| 0usize);

	let restart = {
		let current_game = current_game.clone();
		move |_| current_game.set(AppState::new())
	};

	let test = {
		let current_game = current_game.clone();
		move |_| {
			current_game.set(AppState::test(&SlayCardSpec::HeroCard(
				HeroAbilityType::PlunderingPuma,
			)))
		}
	};

	let choose = {
		let current_game = current_game.clone();
		Callback::from(move |choice_id| current_game.set(current_game.create_new_state(choice_id)))
	};

	html! {
			<>
					<h1>{ "Here to Slay!" }</h1>
					<div>
							<button class={classes!("border-blink")} onclick={restart}>{ "Restart" } </button>
							<button class={classes!("border-blink")} onclick={test}>{ "Test next" } </button>
							<GamePerspectiveView
								game={Rc::new(current_game.perspective())}
								statics={Rc::new(current_game.get_statics())}
								choose={choose.to_owned()}
							/>
					</div>
			</>
	}
}

pub fn render() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::Renderer::<App>::new().render();
}
