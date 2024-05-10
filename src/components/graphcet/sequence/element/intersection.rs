use crate::components::graphcet::sequence::{
  element::{step::StepProps, transition::TransitionProps, Element},
  SequenceProps,
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
