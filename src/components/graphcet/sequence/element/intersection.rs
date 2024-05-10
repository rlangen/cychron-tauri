use crate::{
  components::graphcet::sequence::{
    element::{
      intersection::{
        alternative_intersection::AlternativeIntersection, loop_intersection::LoopIntersection,
        parallel_intersection::ParallelIntersection,
      },
      step::StepProps,
      transition::TransitionProps,
      Element,
    },
    SequenceProps,
  },
  services::uuid_service::UuidService,
};
use yew::prelude::*;

pub(crate) mod alternative_intersection;
pub(crate) mod loop_intersection;
pub(crate) mod parallel_intersection;

#[derive(Clone, PartialEq, Debug)]
pub enum IntersectionType {
  ParallelBranches(Branches, TransitionProps),
  AlternativeBranches(Branches),
  /// First transition exits loop, second continues loop
  LoopBranches(SequenceProps, TransitionProps, TransitionProps),
}
#[derive(Clone, PartialEq, Debug)]
pub struct Branches(pub Vec<SequenceProps>);

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct IntersectionProps {
  pub intersection_type: IntersectionType,
  pub id: u128,

  pub on_insert_element_pair_after_intersection: Callback<IntersectionId>,
}
impl Default for IntersectionType {
  fn default() -> Self {
    IntersectionType::ParallelBranches(
      Branches(vec![
        SequenceProps {
          elements: vec![
            Element::Step(StepProps::default()),
            Element::Transition(TransitionProps::default()),
          ],
          on_insert_element_pair: Callback::noop(),
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_alternative_intersection: Callback::noop(),
          on_insert_element_pair_after_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
        },
        SequenceProps {
          elements: vec![
            Element::Step(StepProps::default()),
            Element::Transition(TransitionProps::default()),
          ],
          on_insert_element_pair: Callback::noop(),
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_alternative_intersection: Callback::noop(),
          on_insert_element_pair_after_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
        },
      ]),
      TransitionProps::default(),
    )
  }
}

#[derive(Copy, Clone)]
pub struct BranchIndex(pub usize);
#[derive(Copy, Clone)]
pub struct TransitionId(pub u128);
#[derive(Copy, Clone)]
pub struct AddToLeft(pub bool);
#[derive(Copy, Clone)]
pub struct IntersectionId(pub u128);

pub struct Intersection {}

impl Component for Intersection {
  type Message = ();
  type Properties = IntersectionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }
  fn view(&self, ctx: &Context<Self>) -> Html {
    let intersection_id = IntersectionId(ctx.props().id);
    html! {
      <div class= "intersection__top-level-container">
      {match &ctx.props().intersection_type {
        IntersectionType::ParallelBranches(in_branches, exit_transition) => html! {
          <ParallelIntersection
            branches={in_branches.0.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}
            on_insert_element_pair_after_intersection={
              ctx
              .props()
              .on_insert_element_pair_after_intersection
              .reform(move |_| intersection_id)
            }/>
        },
        IntersectionType::AlternativeBranches(in_branches) => html! {
          <AlternativeIntersection
            branches={in_branches.0.clone()}
            id={UuidService::new_index()}/>
        },
        IntersectionType::LoopBranches(sequence, continue_transition, exit_transition) => html! {
          <LoopIntersection
            sequence={sequence.clone()}
            continue_transition={continue_transition.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}/>
        },
      }}
    </div>}
  }
}
