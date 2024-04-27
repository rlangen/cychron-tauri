use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct HoverControlProps {
  pub id: u128,
  pub on_add_step: Callback<()>,
  pub on_add_parallel_intersection: Callback<()>,
}

pub struct HoverControl;

impl Component for HoverControl {
  type Message = ();
  type Properties = HoverControlProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="hover_control__container">
        <div class= "hover_control__add-direction hover_control__add-direction--right">
          {"→"}
        </div>
        <div class= "hover_control__add-direction hover_control__add-direction--down">
          {"↓"}
        </div>
        <button
          class="hover_control__button hover_control__button--add-parallel"
          onclick={ctx.props().on_add_parallel_intersection.reform(|_| ())}>
          { "P" }
        </button>
        <button class="hover_control__button hover_control__button--add-alternative">
          { "A" }
        </button>
        <button
          class="hover_control__button hover_control__button--add-step"
          onclick={ctx.props().on_add_step.reform(|_| ())}>
          { "S" }
        </button>
        <button class="hover_control__button hover_control__button--add-delete">
          { "🗑" }
        </button>
      </div>
    }
  }
}
