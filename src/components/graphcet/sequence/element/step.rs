use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub struct Step {
  action_name: String,
}

pub enum Msg {
  ActionNameUpdateEvent(String),
}

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct StepProps {
  pub id: u128,
  pub action_name: String,
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
    html! {
      <div class="step__container">
        <div class="step__number-field">
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
