use crate::{
  components::{net_button::NetButtonProps, net_user_control::NetUserControl},
  services::uuid_service::UuidService,
};
use web_sys::HtmlTextAreaElement;
use yew::{html::IntoPropValue, prelude::*};

pub struct Transition {
  transitions: String,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct TransitionProps {
  pub id: u128,
  pub transitions: String,
  pub buttons: Vec<NetButtonProps>,
}
impl IntoPropValue<String> for TransitionProps {
  fn into_prop_value(self) -> String {
    self.transitions
  }
}
impl Default for TransitionProps {
  fn default() -> Self {
    Self {
      id: UuidService::new_index(),
      transitions: String::from(""),
      buttons: vec![],
    }
  }
}

pub enum Msg {
  TransitionsUpdateEvent(String),
}

impl Component for Transition {
  type Message = Msg;
  type Properties = TransitionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Transition {
      transitions: _ctx.props().transitions.clone(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::TransitionsUpdateEvent(new_text) => {
        self.transitions = new_text;
        return true;
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="transition__container">
        <div class="transition__vertContainer">
          <div class="path__short"/>
          <div class="transition__bar"/>
          <div class="path__short"/>
          <NetUserControl buttons={ctx.props().buttons.clone()}/>
        </div>
        <div class="transition__name-field">
          <textarea class="transition__name-field-text"
            spellcheck="false"
            value={self.transitions.clone()}
            oninput={ctx.link().callback(|e: InputEvent| {
              let input: HtmlTextAreaElement = e.target_unchecked_into();
              Msg::TransitionsUpdateEvent(input.value())
            })}/>
        </div>
      </div>
    }
  }
}
