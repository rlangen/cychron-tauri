use yew::prelude::*;

use crate::{
  components::graphcet::sequence::{
    element::{
      intersection::{BranchIndex, TransitionId},
      transition::{Transition, TransitionProps},
      Element,
    },
    hover_control::HoverControl,
    Sequence, SequenceProps,
  },
  services::uuid_service::UuidService,
};

#[derive(Clone, PartialEq, Properties, Debug)]
pub(super) struct ParallelIntersecionProps {
  pub exit_transition: TransitionProps,
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_append_transition_and_step: Callback<BranchIndex>,
  pub on_add_step_and_transition: Callback<(BranchIndex, TransitionId)>,
}

impl Default for ParallelIntersecionProps {
  fn default() -> Self {
    Self {
      exit_transition: TransitionProps::default(),
      id: UuidService::new_index(),
      branches: vec![SequenceProps {
        elements: vec![
          Element::Step(Default::default()),
          Element::Transition(Default::default()),
        ],
        on_add_step_and_transition: Callback::noop(),
      }],
      on_append_transition_and_step: Callback::noop(),
      on_add_step_and_transition: Callback::noop(),
    }
  }
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
      <div class="intersection__parallel-branch-seperation-line"/>
      <div class="intersection__grid-container" key={ctx.props().id.to_string()+"_grid-container"}>
        {for ctx.props().branches.iter().enumerate().map(|(index, item)| {
          html! {
            <div class="intersection__grid-item">
              <div class="intersection__content-wrapper">
                <div class="path__short path__short--margin-left"/>
                  <Sequence
                    elements={item.elements.clone()}
                    on_add_step_and_transition={
                      ctx
                      .props().on_add_step_and_transition
                      .reform(move |transition_id| (BranchIndex(index), transition_id))}/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="intersection__hover-container">
                <div class="path__short path__short--margin-left"/>
                <HoverControl
                  on_add_step={ctx.props().on_append_transition_and_step.reform(move |_| BranchIndex(index))}
                  id={UuidService::new_index()}/>
              </div>
            </div>
          }
        })}
      </div>
      <div class="intersection__parallel-branch-seperation-line"/>
      <Transition
        transitions={ctx.props().exit_transition.transitions.clone()}
        id={ctx.props().exit_transition.id.clone()}
        on_add_step={ctx.props().exit_transition.on_add_step.clone()}/>
    </>}
  }
}
