use yew::prelude::*;

use crate::components::{
  component_tuple_structs::NeedsRerendering,
  graphcet::sequence::{
    element::{
      intersection::{AddToLeft, BranchIndex, IntersectionId, TransitionId},
      step::StepProps,
      transition::TransitionProps,
      Element, StepId,
    },
    Sequence, SequenceProps,
  },
  net_button::{NetButtonDirection, NetButtonProps},
  net_user_control::NetUserControl,
};

use super::{IntersectionProps, IntersectionType};
// pub mod alternative_intersection_behaviour;

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct AlternativeIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_add_branch: Callback<(BranchIndex, AddToLeft)>,

  pub on_prepend_element_pair: Callback<BranchIndex>,
  pub on_append_element_pair: Callback<(BranchIndex, TransitionId)>,

  pub on_pass_attach_element_pair_to_intersection: Callback<(BranchIndex, IntersectionId)>,

  pub on_insert_parallel_intersection: Callback<(BranchIndex, StepId)>,
  pub on_insert_alternative_intersection: Callback<(BranchIndex, TransitionId)>,
  pub on_insert_loop_intersection: Callback<(BranchIndex, StepId)>,
}

pub(crate) struct AlternativeIntersection;
impl AlternativeIntersection {
  pub fn add(sequence: &mut SequenceProps, transition_id: TransitionId) -> NeedsRerendering {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| transition_id.0 == x.get_id())
    {
      let entry_transition = sequence.elements.remove(pos);
      let step = sequence.elements.remove(pos);
      let exi_transition = sequence.elements.remove(pos);

      let new_alternative_intersection = Element::Intersection(IntersectionProps {
        id: 0,
        intersection_type: IntersectionType::AlternativeBranches,
        branches: vec![
          SequenceProps {
            elements: vec![entry_transition, step, exi_transition],
            on_insert_element_pair: Callback::noop(),
            on_insert_parallel_intersection: Callback::noop(),
            on_insert_alternative_intersection: Callback::noop(),
            on_attach_element_pair_to_intersection: Callback::noop(),
            on_insert_loop_intersection: Callback::noop(),
          },
          SequenceProps {
            elements: vec![
              Element::Transition(TransitionProps::default()),
              Element::Step(StepProps::default()),
              Element::Transition(TransitionProps::default()),
            ],
            on_insert_element_pair: Callback::noop(),
            on_insert_parallel_intersection: Callback::noop(),
            on_insert_alternative_intersection: Callback::noop(),
            on_attach_element_pair_to_intersection: Callback::noop(),
            on_insert_loop_intersection: Callback::noop(),
          },
        ],
        on_attach_element_pair_to_intersection: Callback::noop(),
      });

      sequence.elements.insert(pos, new_alternative_intersection);

      return NeedsRerendering(true);
    } else {
      return NeedsRerendering(false);
    }
  }
}

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
                  on_insert_element_pair={
                    ctx
                    .props()
                    .on_append_element_pair
                    .reform(move |transition_id| (BranchIndex(index), transition_id))
                  }
                  on_insert_parallel_intersection={
                    ctx
                    .props()
                    .on_insert_parallel_intersection
                    .reform(move |step_id| (BranchIndex(index), step_id))
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
      </div>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
      <div class="path__short path__short--margin-left"/>
    </>}
  }
}
