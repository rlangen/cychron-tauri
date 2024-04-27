use yew::prelude::*;

use crate::components::net_button::{NetButton, NetButtonDirection};

#[derive(Clone, PartialEq, Properties)]
pub struct StepHoverMenuProps {
  pub on_add_parallel_intersection: Callback<()>,
}

pub struct StepHoverMenu;

impl Component for StepHoverMenu {
  type Message = ();
  type Properties = StepHoverMenuProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="net-hover-menu__container">
        <NetButton
          direction={Some(NetButtonDirection::East)}
          button_text={"P".to_string()}
          on_click={ctx.props().on_add_parallel_intersection.reform(|_| ())}/>
      </div>
    }
  }
}
