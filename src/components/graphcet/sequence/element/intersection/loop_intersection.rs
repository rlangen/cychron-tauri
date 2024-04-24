use yew::prelude::*;

use crate::components::graphcet::sequence::{
  element::{
    intersection::parallel_intersection::OnAddStepAndTransitionData,
    transition::{Transition, TransitionProps},
  },
  Sequence, SequenceProps,
};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct LoopIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub continue_transition: TransitionProps,
  pub exit_transition: TransitionProps,
  pub on_append_step_and_transition: Callback<OnAddStepAndTransitionData>,
}

pub(super) struct LoopIntersection;

impl Component for LoopIntersection {
  type Message = ();

  type Properties = LoopIntersectionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {<>
      <div
        key={ctx.props().id}
        class="intersection__loop-branch-seperation-line"/>
      <div class="intersection__grid-container">
        {for ctx.props().branches.iter().enumerate().map(|(index, item)| {
          html! {
            <div class="intersection__grid-item">
              <div class="intersection__content-wrapper">
                <div class="path__short path__short--margin-left"/>
                <Sequence
                  key={index.clone()}
                  elements={item.elements.clone()}
                  on_add_step_and_transition={ctx.props().on_append_step_and_transition.reform(move |transition_id| OnAddStepAndTransitionData{
                    branch_index: index,
                    transition_id,
                  })}/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="path__short path__short--margin-left"/>
            </div>
          }
        })}
        <div class="intersection__grid-item">
          <div class="path__short path__short--margin-left"/>
          <div class="path__dynamic"/>
          <Transition
            transitions={ctx.props().continue_transition.transitions.clone()}
            id={ctx.props().continue_transition.id.clone()}
            on_add_step={ctx.props().continue_transition.on_add_step.clone()}/>
          <div class="path__triangle_arrow_up"/>
          <div class="path__short path__short--margin-left"/>
        </div>
      </div>
      <div
        key={ctx.props().id}
        class="intersection__loop-branch-seperation-line"/>
        <Transition
          transitions={ctx.props().exit_transition.clone()}
          id={ctx.props().exit_transition.id.clone()}
          on_add_step={ctx.props().exit_transition.on_add_step.clone()}/>
    </>}
  }
}
