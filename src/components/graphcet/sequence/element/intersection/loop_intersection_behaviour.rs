use crate::components::graphcet::sequence::{element::Element, SequenceProps};

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
}
