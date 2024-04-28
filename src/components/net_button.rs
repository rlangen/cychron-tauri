use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct NetButtonProps {
  pub direction: Option<NetButtonDirection>,
  pub button_text: String,
  pub on_click: Callback<()>,
}
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum NetButtonDirection {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}
impl NetButtonDirection {
  #[allow(dead_code)]
  pub fn as_char(&self) -> char {
    match self {
      NetButtonDirection::North => '⭡',
      NetButtonDirection::NorthEast => '⭷',
      NetButtonDirection::East => '⭢',
      NetButtonDirection::SouthEast => '⭸',
      NetButtonDirection::South => '⭣',
      NetButtonDirection::SouthWest => '⭹',
      NetButtonDirection::West => '⭠',
      NetButtonDirection::NorthWest => '⭶',
    }
  }
}

pub struct NetButton;

impl Component for NetButton {
  type Message = ();

  type Properties = NetButtonProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <button class="net_button__container"
        onclick={ctx.props().on_click.reform(|_| ())}>
        {if let Some(direction_arrow) = &ctx.props().direction {
          html!{<>
            <div class="net_button__direction">
                {direction_arrow.as_char()}
            </div>
            <div class="net_button__direction-seperation-line"/>
          </>}
        } else {
          html!{}
        }}
        <div
            class="net_button__action-type">
          {&ctx.props().button_text}
        </div>
      </button>
    }
  }
}
