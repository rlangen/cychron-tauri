use ::yew::prelude::*;

use crate::{
  components::{
    component_tuple_structs::NeedsRerendering,
    graphcet::sequence::{
      element::{transition::TransitionProps, Element, StepId},
      SequenceProps,
    },
  },
  services::logging_service::Log,
};

use super::{IntersectionProps, IntersectionType};

pub struct LoopIntersectionBehaviour;
impl LoopIntersectionBehaviour {
  pub fn should_be_viewed(sequence: &SequenceProps, triggering_element: u128) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| triggering_element == x.get_id())
    {
      match sequence.elements.get(pos) {
        Some(Element::Step(_)) => {}
        _ => return false,
      }
      match sequence.elements.get(pos + 1) {
        Some(Element::Transition(_)) => {}
        _ => return false,
      }
      return true;
    } else {
      return false;
    }
  }

  pub fn add(sequence: &mut SequenceProps, step_id: StepId) -> NeedsRerendering {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| step_id.0 == x.get_id())
    {
      let step = sequence.elements.remove(pos);
      let exit_transition = match sequence.elements.remove(pos) {
        Element::Transition(transition_props) => transition_props,
        _ => {
          Log::error::<Self>(
            "The step to add loop intersection does not have a following transition.",
          );
          return NeedsRerendering(false);
        }
      };

      let new_loop_intersection = Element::Intersection(IntersectionProps {
        id: 0,
        intersection_type: IntersectionType::LoopBranches(
          TransitionProps::default(),
          exit_transition,
        ),
        branches: vec![SequenceProps {
          elements: vec![step],
          on_insert_element_pair: Callback::noop(),
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_alternative_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
          on_attach_element_pair_to_intersection: Callback::noop(),
        }],
        on_attach_element_pair_to_intersection: Callback::noop(),
      });

      sequence.elements.insert(pos, new_loop_intersection);
      return NeedsRerendering(true);
    } else {
      Log::error::<Self>("Step to add loop intersection not found");
      return NeedsRerendering(false);
    }
  }
}
