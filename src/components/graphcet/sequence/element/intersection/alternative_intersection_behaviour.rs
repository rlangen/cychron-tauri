use crate::components::graphcet::sequence::{self, element::Element};

pub struct AlternativeIntersectionBehaviour;
impl AlternativeIntersectionBehaviour {
  pub fn should_be_viewed(sequence: &sequence::SequenceProps, triggering_element: u128) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| triggering_element == x.get_id())
    {
      match sequence.elements.get(pos) {
        Some(Element::Transition(_)) => {}
        _ => return false,
      }
      match sequence.elements.get(pos + 1) {
        Some(Element::Step(_)) => {}
        _ => return false,
      }
      match sequence.elements.get(pos + 2) {
        Some(Element::Transition(_)) => {}
        _ => return false,
      }
      return true;
    } else {
      return false;
    }
  }
}
