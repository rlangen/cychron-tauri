use yew::prelude::*;

use crate::components::net_button::{NetButton, NetButtonProps};

#[derive(Clone, PartialEq, Properties)]
pub struct NetUserControlProps {
  pub buttons: Vec<NetButtonProps>,
}

pub struct NetUserControl;
impl Component for NetUserControl {
  type Message = ();
  type Properties = NetUserControlProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="net-hover-menu__container">
        {for ctx.props().buttons.iter().map(|button| {
          html! {
            <NetButton
              direction={button.direction.clone()}
              button_text={button.button_text.clone()}
              on_click={button.on_click.clone()}/>
          }
        })}
      </div>
    }
  }
}
