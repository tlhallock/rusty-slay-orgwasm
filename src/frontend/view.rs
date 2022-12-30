use yew::prelude::*;

use crate::frontend::app::AppState;
use crate::frontend::game::GamePerspectiveView;

// let (player_id, choice_id) = strategy::pick_a_random_choice(context, game)?;

#[function_component(App)]
fn app() -> Html {
	let current_game = use_state(|| AppState::new());
	let _player_index = use_state(|| 0usize);

	let restart = {
		let current_game = current_game.clone();
		move |_| current_game.set(AppState::new())
	};

	let choose = {
		let current_game = current_game.clone();
		Callback::from(move |choice_id| current_game.set(current_game.create_new_state(choice_id)))
	};

	html! {
			<>
					<h1>{ "Hello World" }</h1>
					<div>
							<button onclick={restart}>{ "Restart" } </button>
							<GamePerspectiveView game={current_game.perspective()} choose={choose.to_owned()}/>
					</div>
			</>
	}
}

pub fn render() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::Renderer::<App>::new().render();
}
