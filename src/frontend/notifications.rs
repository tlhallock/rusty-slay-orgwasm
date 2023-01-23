use std::collections::VecDeque;
use std::rc::Rc;

use yew::classes;
use yew::prelude::*;

use crate::frontend::app::CommonProps;
use crate::frontend::deck::DeckView;
use crate::slay::ids;
use crate::slay::notification::Notification;
use crate::slay::state::game::GameStaticInformation;
use crate::slay::state::player::PlayerPerspective;


#[derive(Properties, PartialEq)]
pub struct NotificationProps {
  pub notifications: VecDeque<Notification>,
	pub statics: Rc<GameStaticInformation>,
  pub player_index: ids::PlayerIndex,
}

#[function_component(Notifications)]
pub fn view_notifications(props: &NotificationProps) -> Html {
  // Dry:
  let notifications = props.notifications.iter()
    .map(|notification| 
      html! {
        <div>
          {notification.get_description(&(*props.statics), props.player_index)}
        </div>
      }
    );
  let num_notifications = props.notifications.len();
  let last_notifications = props.notifications.iter()
    .enumerate().filter(|(idx, _)| *idx >= num_notifications - 4)
    .map(|(_, notification)| 
      html! {
        <div>
          {notification.get_description(&(*props.statics), props.player_index)}
        </div>
      }
    );
	// Does this need to be one higher?
	// TODO: DRY?
	let is_open = use_state(|| false);
	let close = {
		let open_handle = is_open.clone();
		Callback::from(move |_| open_handle.set(false))
	};
	let open = {
		let open_handle = is_open.clone();
		Callback::from(move |_| open_handle.set(true))
	};
	if !*is_open {
		return html! {
      <div>
        { for last_notifications }
  			<button onclick={open}>
  				{ "+" }
  			</button>
      </div>
		};
	}
	html! {
			<div>
					{ for notifications }
          <br/>
          <div onclick={close}>
            <img
              src={"imgs/icons/back.png"}
              alt={"Go back"}
              width={50}
            />
          </div>
			</div>
	}
}