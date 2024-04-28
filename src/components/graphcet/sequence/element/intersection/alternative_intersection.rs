use yew::prelude::*;

use crate::components::{
  graphcet::sequence::{
    element::{
      intersection::{BranchIndex, TransitionId},
      StepId,
    },
    Sequence, SequenceProps,
  },
  net_button::{NetButtonDirection, NetButtonProps},
  net_user_control::NetUserControl,
};

use super::AddToLeft;

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct AlternativeIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_add_branch: Callback<(BranchIndex, AddToLeft)>,
  pub on_prepend_element_pair: Callback<BranchIndex>,
  pub on_append_step_and_transition: Callback<(BranchIndex, TransitionId)>,
  pub on_add_parallel_intersection: Callback<(BranchIndex, StepId)>,
}

pub(super) struct AlternativeIntersection;

impl Component for AlternativeIntersection {
  type Message = ();

  type Properties = AlternativeIntersectionProps;

  fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
    html! {<>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
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
                        direction: Some(NetButtonDirection::West),
                        button_text: "B".to_string(),
                        on_click: ctx.props().on_add_branch.reform(move |_| (BranchIndex(index), AddToLeft(true))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::South),
                        button_text: "S".to_string(),
                        on_click: ctx.props().on_prepend_element_pair.reform(move |_| BranchIndex(index)),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::East),
                        button_text: "B".to_string(),
                        on_click: ctx.props().on_add_branch.reform(move |_| (BranchIndex(index), AddToLeft(false))),
                      },
                    ]}/>
                </div>
                <Sequence
                  key={index.clone()}
                  elements={item.elements.clone()}
                  on_add_step_and_transition={
                    ctx
                    .props()
                    .on_append_step_and_transition
                    .reform(move |transition_id| (BranchIndex(index), transition_id))
                  }
                  on_add_parallel_intersection={
                    ctx
                    .props()
                    .on_add_parallel_intersection
                    .reform(move |step_id| (BranchIndex(index), step_id))
                  }/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="path__short path__short--margin-left"/>
            </div>
          }
        })}
      </div>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
      <div class="path__short path__short--margin-left"/>
    </>}
  }
}
