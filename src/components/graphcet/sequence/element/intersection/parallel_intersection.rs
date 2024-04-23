use yew::prelude::*;

use crate::{
  components::graphcet::sequence::{
    element::transition::{Transition, TransitionProps},
    hover_control::HoverControl,
    Sequence, SequenceProps,
  },
  services::uuid_service::UuidService,
};

#[derive(Clone, PartialEq, Properties, Debug)]
pub(super) struct ParallelIntersecionProps {
  pub exit_transition: TransitionProps,
  pub line_width: usize,
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_append_transition_and_step: Callback<usize>,
  pub on_add_step_and_transition: Callback<OnAddStepAndTransitionData>,
}
pub struct OnAddStepAndTransitionData {
  pub branch_index: usize,
  pub transition_id: u128,
}

pub(super) struct ParallelIntersection;

impl Component for ParallelIntersection {
  type Message = ();
  type Properties = ParallelIntersecionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {<>
      <div
        style={format!("
        height: 2px;
        width: {}px;
        background-color: black;", ctx.props().line_width)}/>
      <div style="height: 2px;"/>
      <div
        style={format!("
        height: 2px;
        width: {}px;
        background-color: black;", ctx.props().line_width)}/>
      <div class="intersection__grid-container" key={ctx.props().id.to_string()+"_grid-container"}>
        {for ctx.props().branches.iter().enumerate().map(|(index, item)| {
          html! {
            <div class="intersection__grid-item">
              <div class="intersection__content-wrapper">
                <div class="path__short path__short--margin-left"/>
                  <Sequence
                    elements={item.elements.clone()}
                    on_add_step_and_transition={ctx.props().on_add_step_and_transition.reform(move |transition_id| OnAddStepAndTransitionData {
                    branch_index: index,
                    transition_id,})}/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="intersection__hover-container">
                <div class="path__short path__short--margin-left"/>
                <HoverControl
                  on_add_step={ctx.props().on_append_transition_and_step.reform(move |_| index)}
                  id={UuidService::new_index()}/>
              </div>
            </div>
          }
        })}
      </div>
      <div
        style={format!("
        height: 2px;
        width: {}px;
        background-color: black;", ctx.props().line_width)}/>
      <div style="height: 2px;"/>
      <div
        style={format!("
        height: 2px;
        width: {}px;
        background-color: black;", ctx.props().line_width)}/>
      <Transition
        transitions={ctx.props().exit_transition.transitions.clone()}
        id={ctx.props().exit_transition.id.clone()}
        on_add_step={ctx.props().exit_transition.on_add_step.clone()}/>
    </>}
  }
}
