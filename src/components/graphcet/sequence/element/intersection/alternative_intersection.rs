use yew::prelude::*;

use crate::components::graphcet::sequence::{
  element::intersection::parallel_intersection::OnAddStepAndTransitionData, Sequence, SequenceProps,
};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct AlternativeIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_append_step_and_transition: Callback<OnAddStepAndTransitionData>,
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
      </div>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
      <div class="path__short path__short--margin-left"/>
    </>}
  }
}
