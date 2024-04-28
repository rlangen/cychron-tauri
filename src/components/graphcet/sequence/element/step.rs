use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
  components::{
    graphcet::sequence::element::StepId,
    net_button::{NetButtonDirection, NetButtonProps},
    net_user_control::NetUserControl,
  },
  services::uuid_service::UuidService,
};

pub struct Step {
  action_name: String,
}

pub enum Msg {
  ActionNameUpdateEvent(String),
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct StepProps {
  pub id: u128,
  pub action_name: String,
  pub on_insert_parallel_intersection: Callback<StepId>,
}
impl Default for StepProps {
  fn default() -> Self {
    Self {
      id: UuidService::new_index(),
      action_name: String::from(""),
      on_insert_parallel_intersection: Callback::noop(),
    }
  }
}

impl Component for Step {
  type Message = Msg;
  type Properties = StepProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      action_name: _ctx.props().action_name.clone(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::ActionNameUpdateEvent(new_text) => {
        self.action_name = new_text;
        return true;
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let id = StepId(ctx.props().id);
    html! {
      <div class="step__container">
        <div class="step__number-field">
          <NetUserControl
            buttons={vec![
              NetButtonProps {
                direction: Some(NetButtonDirection::East),
                button_text: "P".to_string(),
                on_click: ctx.props().on_insert_parallel_intersection.reform(move |_| id),
              },
            ]}/>
          // {ctx.props().id}
        </div>
        <div class="step__horizontal-connector"/>
        <div class="action__container">
          <textarea
            class="action__textarea"
            spellcheck="false"
            value={self.action_name.clone()}
            oninput={ctx.link().callback(|e: InputEvent| {
              let input: HtmlTextAreaElement = e.target_unchecked_into();
              Msg::ActionNameUpdateEvent(input.value())
            })}/>
        </div>
      </div>
    }
  }
}
