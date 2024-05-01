use yew::prelude::*;

use crate::components::{
  graphcet::sequence::{
    element::{
      intersection::{BranchIndex, IntersectionId, TransitionId},
      transition::{Transition, TransitionProps},
      StepId,
    },
    Sequence, SequenceProps,
  },
  net_button::{NetButtonDirection, NetButtonProps},
  net_user_control::NetUserControl,
};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct LoopIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub continue_transition: TransitionProps,
  pub exit_transition: TransitionProps,

  pub on_prepend_element_pair: Callback<BranchIndex>,
  pub on_append_element_pair: Callback<(BranchIndex, TransitionId)>,

  pub on_pass_attach_element_pair_to_intersection: Callback<(BranchIndex, IntersectionId)>,

  pub on_insert_parallel_intersection: Callback<(BranchIndex, StepId)>,
  pub on_insert_alternative_intersection: Callback<(BranchIndex, TransitionId)>,
  pub on_insert_loop_intersection: Callback<(BranchIndex, StepId)>,
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
                <div class="
                  path__short 
                  path__short--margin-left 
                  intersection__entry-menu">
                  <NetUserControl
                    buttons={vec![
                      NetButtonProps {
                        direction: Some(NetButtonDirection::South),
                        button_text: "S".to_string(),
                        on_click: ctx.props().on_prepend_element_pair.reform(move |_| BranchIndex(index)),
                      },
                    ]}/>
                </div>
                <Sequence
                  key={index.clone()}
                  elements={item.elements.clone()}
                  on_insert_element_pair={
                    ctx
                    .props()
                    .on_append_element_pair
                    .reform(move |transition_id| (BranchIndex(index), transition_id),)
                  }
                  on_insert_parallel_intersection={
                    ctx
                    .props()
                    .on_insert_parallel_intersection
                    .reform(move |step_id| (BranchIndex(index), step_id),)
                  }
                  on_attach_element_pair_to_intersection={
                    ctx
                    .props()
                    .on_pass_attach_element_pair_to_intersection
                    .reform(move |intersection_id| (BranchIndex(index), intersection_id))
                  }
                  on_insert_alternative_intersection={
                    ctx
                    .props()
                    .on_insert_alternative_intersection
                    .reform(move |transition_id| (BranchIndex(index), transition_id))
                  }
                  on_insert_loop_intersection={
                    ctx
                    .props()
                    .on_insert_loop_intersection
                    .reform(move |step_id| (BranchIndex(index), step_id))
                  }/>
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
            buttons={vec![]}/>
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
          buttons={vec![]}/>
    </>}
  }
}
