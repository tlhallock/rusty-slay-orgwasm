use yew::classes;
use yew::prelude::*;

use log;

use crate::common::perspective::ChoiceAssociationType;
use crate::common::perspective::StackPerspective;

use super::app::ChoiceState;
use super::card_modal::CardModalInfo;

#[derive(Properties, PartialEq)]
pub struct StackProps {
    pub stack: StackPerspective,
    pub view_card: Callback<Option<CardModalInfo>, ()>,
    pub choice_state: ChoiceState,
}

impl StackProps {
    fn get_css_class(&self) -> Option<&'static str> {
        let has_association = self.stack.top.choice_associations.len() > 0;
        if !has_association {
            return None;
        }

        log::info!("has an association.");

        let is_main_option = self
            .stack
            .top
            .choice_associations
            .iter()
            .any(|a| a.association_type == ChoiceAssociationType::Representer);

        if is_main_option {
            return Some("is_choice");
        }

        let _is_highlighted = self.choice_state.highlighted_choice.iter().any(|id| {
            self.stack
                .top
                .choice_associations
                .iter()
                .any(|a| a.choice_id == *id)
        });
        let _is_default = self
            .stack
            .top
            .choice_associations
            .iter()
            .any(|a| a.is_default);

        return Some("is-part-of-choice");
    }
}

#[function_component(StackView)]
pub fn view_stack(props: &StackProps) -> Html {
    let view_this_card = {
        let view_card = props.view_card.clone();
        let modal = props.stack.top.to_card_modal();
        move |_| view_card.emit(Some(modal.clone()))
    };

    let c = props.get_css_class();

    html! {
        <div
            class={classes!("card", c)}
            onclick={view_this_card}
        >
            { props.stack.top.spec.label.to_owned() }
        </div>
    }
}
