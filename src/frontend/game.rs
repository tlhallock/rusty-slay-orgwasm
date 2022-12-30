use yew::classes;
use yew::prelude::*;

use log;

use crate::common::perspective::GamePerspective;
use crate::frontend::app::ChoiceState;
use crate::frontend::card_modal::CardModalInfo;
use crate::frontend::card_modal::CardModalView;
use crate::frontend::choices::ChoicesView;
use crate::frontend::deck::DeckView;
use crate::frontend::player::PlayerView;
use crate::frontend::roll_modal::RollModalView;
use crate::slay::ids;

#[derive(Properties, PartialEq)]
pub struct GamePerspectiveProps {
    pub game: GamePerspective,
    pub choose: Callback<ids::ChoiceId, ()>,
}

#[function_component(GamePerspectiveView)]
pub fn view_game(props: &GamePerspectiveProps) -> Html {
    let viewed_card = use_state(|| None::<CardModalInfo>);
    let view_card = {
        let viewed_card = viewed_card.clone();
        Callback::from(move |m| viewed_card.set(m))
    };
    let choice_state = use_state(|| ChoiceState::default());
    let set_selected_choice = {
        let choice_state = choice_state.clone();
        Callback::from(move |highlighted_choice| {
            log::info!("Trying to set highlight");
            choice_state.set(ChoiceState { highlighted_choice })
        })
    };

    let rotated = props.game.rotated_players();
    let players = rotated.iter().map(|player| {
        html! {
            <PlayerView
                player={(*player).to_owned()}
                view_card={view_card.to_owned()}
                choice_state={(*choice_state).to_owned()}
            />
        }
    });
    let decks = props.game.decks.iter().map(|deck| {
        html! {
            <DeckView
                deck={deck.to_owned()}
                view_card={view_card.to_owned()}
                choice_state={(*choice_state).to_owned()}
            />
        }
    });
    let card_view = viewed_card.as_ref().map(|m| {
        html! {
            <CardModalView info={m.to_owned()} view_card={view_card.to_owned()} />
        }
    });
    let choices_instructions = props.game.choices.as_ref().map(|c| {
        html! {
            <ChoicesView
                choices={c.to_owned()}
                choose={props.choose.to_owned()}
                set_selected_choice={set_selected_choice.to_owned()}
            />
        }
    });
    let roll = props.game.roll.as_ref().map(|r| {
        html! {
            <RollModalView roll={r.to_owned()}/>
        }
    });
    html! {
        <div>
            {for choices_instructions}
            {for card_view}
            {for roll}
            <div class={classes!("global-decks")}>
                {for decks}
            </div>
            <div class={classes!("players")}>
                {for players}
            </div>
        </div>
    }
}
