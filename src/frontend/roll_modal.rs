use yew::classes;
use yew::prelude::*;

use log;

use crate::common::perspective::RollPerspective;
use crate::slay::showdown::completion::RollCompletion;
use crate::slay::showdown::consequences::Comparison;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::roll_state::RollReason;
use crate::slay::tasks::TaskSpec;

fn format_condition(condition: &Condition) -> String {
  match condition.cmp {
    Comparison::LE => format!("<= {}", condition.threshold),
    Comparison::GE => format!(">= {}", condition.threshold),
  }
}


fn format_consequences(consequences: &Vec<TaskSpec>) -> String {
  consequences.iter().map(
    |spec| match spec {
        TaskSpec::Sacrifice(num) => format!("Sacrifice {} heros.", num),
        TaskSpec::Discard(num) => format!("Discard {} cards.", num),
        // These are not the bad things...
        TaskSpec::ReceiveModifier(_) => unreachable!(),
        TaskSpec::Draw(_) => unreachable!(),
    }
  ).collect()
}



#[function_component(RollDescription)]
pub fn view_roll_context(props: &RollModalProps) -> Html {
    let text = match &props.roll.reason {
      RollReason::UseHeroAbility(spec) => html! {
        <>
          {
            format!(
              "{} is rolling for {}'s ability.",
              props.roll.roller_name,
              spec.label,
            )
          }
          <br/>
          {
            format!(
              // TODO: add probability XD
              "If the roll is {}, the player be able to {}.",
              format_condition(&spec.hero_ability.as_ref().unwrap().success_condition),
              spec.description,
            )
          }
          <br/>
        </>
      },
      RollReason::AttackMonster(spec) => html! {
        <>
          {
            format!(
              "{} is rolling to defeat {}!",
              props.roll.roller_name,
              spec.label,
            )
          }
          <br/>
          {
            format!(
              // TODO: add probability XD
              "If the roll is {}, the player be have: {}.",
              format_condition(&spec.monster.as_ref().unwrap().victory.condition),
              spec.description,
            )
          }
          <br/>
          {
            format!(
              "If the roll is {}, the player be have to: {}.",
              format_condition(&spec.monster.as_ref().unwrap().loss.condition),
              format_consequences(&spec.monster.as_ref().unwrap().loss.tasks),
            )
          }
          <br/>
          {
            format!(
              "This player currently has slain {} monsters.",
              "<implement me!>",
            )
          }
          <br/>
        </>
      },
    };
    html! {
      <label>{"The instructions go here."}<br/>{text}</label>
    }
}


#[function_component(RollTotal)]
pub fn view_roll_result(props: &RollModalProps) -> Html {
    html! {
      <label>{format!("The current roll total is {}, which is a {}.",
        props.roll.roll_total,
        if props.roll.success {
          "success"
        } else {
          "failure"
        }
      )}</label>
    }
}
#[function_component(RollTimer)]
fn view_roll_timer(props: &RollModalProps) -> Html {
    html! {
      <label>{format!("This roll times out at {:?}", props.roll.deadline)}</label>
    }
}

#[function_component(RollChoices)]
fn view_roll_choices(props: &RollModalProps) -> Html {
    let open = use_state(|| false);
    html! {
      <label>{"Implement the roll choices."}</label>
    }
}

#[function_component(RollHistory)]
fn view_roll_history(props: &RollModalProps) -> Html {
    let completions = props.roll.history.iter().map(|m| {
        html! {
           <label>
             { format!("Player {} used {} to modify by {}.",
               m.modifyer_name,
               "<implement this>",
               m.modification_amount,
        ) }
           </label>
         }
    });
    html! { <> { for completions } </> }
}
#[function_component(RollCompletions)]
fn view_roll_completions(props: &RollModalProps) -> Html {
    let completions = props.roll.completions.iter().map(|c| {
        html! {
              <label>
                { format!("Player {} is {}.", c.player_name, match c.completion {
                  RollCompletion::Thinking => "thinking",
        RollCompletion::DoneUntilModification => "done for now",
        RollCompletion::AllDone => "done",
        }) }
              </label>
            }
    });
    html! { <> { for completions } </> }
}

#[derive(Properties, PartialEq)]
pub struct RollModalProps {
    pub roll: RollPerspective,
}

#[function_component(RollModalView)]
pub fn view_roll_modal(props: &RollModalProps) -> Html {
    let open = use_state(|| false);

    log::info!("We are creating the modal");

    // let clear_card = {
    //     let view_card = props.view_card.clone();
    //     move |_| view_card.emit(None)
    // };
    html! {
      <div class={classes!("modal")}>
        <div class={classes!("modal-content")}>
          <RollDescription roll={props.roll.to_owned()}/>
          <br/>
          <RollTimer roll={props.roll.to_owned()}/>
          <br/>
          <RollHistory roll={props.roll.to_owned()}/>
          <br/>
          <RollChoices roll={props.roll.to_owned()}/>
          <br/>
          <RollTotal roll={props.roll.to_owned()}/>
          <br/>
          <RollCompletions roll={props.roll.to_owned()}/>
          <br/>
          <button>{"Do not modify this roll, unless someone else does."}</button>
          <br/>
          <button>{"Do not modify this roll"}</button>
          <br/>
          <button>{"Back"}</button>
        </div>
      </div>
    }
}
