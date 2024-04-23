pub mod intersection;
pub mod step;
pub mod transition;

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
  Step(step::StepProps),
  Transition(transition::TransitionProps),
  Intersection(intersection::IntersectionProps),
}
impl Element {
  pub fn get_id(&self) -> u128 {
    match self {
      Element::Step(step_props) => step_props.id,
      Element::Transition(transition_props) => transition_props.id,
      Element::Intersection(intersection_props) => intersection_props.id,
    }
  }
}
