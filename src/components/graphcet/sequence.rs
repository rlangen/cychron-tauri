use std::vec;

use yew::{html::IntoPropValue, prelude::*};

pub mod element;
use element::{
  intersection::{Intersection, IntersectionId, TransitionId},
  step::Step,
  transition::Transition,
  Element,
};

use crate::{
  components::{
    graphcet::sequence::element::intersection::{
      alternative_intersection_behaviour::AlternativeIntersectionBehaviour,
      loop_intersection_behaviour::LoopIntersectionBehaviour,
    },
    net_button::{NetButtonDirection, NetButtonProps},
  },
  services::{logging_service::Log, uuid_service::UuidService},
};

use self::element::{
  intersection::{Branches, IntersectionProps, IntersectionType},
  step::StepProps,
  transition::TransitionProps,
  StepId,
};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct SequenceProps {
  pub elements: Vec<Element>,
  pub on_insert_element_pair: Callback<TransitionId>,
  pub on_insert_parallel_intersection: Callback<StepId>,
  pub on_insert_alternative_intersection: Callback<TransitionId>,
  pub on_insert_loop_intersection: Callback<StepId>,
  pub on_insert_element_pair_after_intersection: Callback<IntersectionId>,
}
impl IntoPropValue<Vec<Element>> for SequenceProps {
  fn into_prop_value(self) -> Vec<Element> {
    self.elements
  }
}
impl Default for SequenceProps {
  fn default() -> Self {
    Self {
      elements: Vec::<Element>::with_capacity(2),
      on_insert_element_pair: Callback::noop(),
      on_insert_element_pair_after_intersection: Callback::noop(),
      on_insert_parallel_intersection: Callback::noop(),
      on_insert_alternative_intersection: Callback::noop(),
      on_insert_loop_intersection: Callback::noop(),
    }
  }
}

pub struct Sequence;
impl Sequence {
  pub fn insert_element_pair(sequence: &mut SequenceProps, transition_id: TransitionId) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| transition_id.0 == x.get_id())
    {
      sequence
        .elements
        .insert(pos + 1, Element::Step(StepProps::default()));
      sequence
        .elements
        .insert(pos + 2, Element::Transition(TransitionProps::default()));

      return true;
    }
    Log::error::<Self>("Failed to add step. Could't find transition to append to.");
    return false;
  }

  pub fn insert_element_pair_after_intersection(
    sequence: &mut SequenceProps,
    intersection_id: IntersectionId,
  ) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| intersection_id.0 == x.get_id())
    {
      let new_step = Element::Step(StepProps::default());
      sequence.elements.insert(pos + 1, new_step);

      let new_transition = Element::Transition(TransitionProps::default());
      sequence.elements.insert(pos + 2, new_transition);

      return true;
    } else {
      Log::error::<Self>("Intersection to add step and transition not found");
      return false;
    }
  }

  pub fn insert_parallel_intersection(sequence: &mut SequenceProps, step_id: StepId) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| step_id.0 == x.get_id())
    {
      let triggering_step = sequence.elements.remove(pos);
      let triggering_transition;
      match sequence.elements[pos] {
        Element::Transition(_) => match sequence.elements.remove(pos) {
          Element::Transition(transition_props) => {
            triggering_transition = transition_props;
          }
          _ => {
            triggering_transition = TransitionProps::default();
          }
        },
        _ => {
          triggering_transition = TransitionProps::default();
        }
      }

      let mut left_branch = SequenceProps::default();
      left_branch.elements.push(triggering_step);

      let mut right_branch = SequenceProps::default();
      right_branch
        .elements
        .push(Element::Step(StepProps::default()));

      let new_parallel_intersection = IntersectionProps {
        id: UuidService::new_index(),
        intersection_type: IntersectionType::ParallelBranches(
          Branches(vec![left_branch, right_branch]),
          triggering_transition,
        ),
        on_insert_element_pair_after_intersection: Callback::noop(),
      };

      sequence
        .elements
        .insert(pos, Element::Intersection(new_parallel_intersection));

      return true;
    } else {
      Log::error::<Self>("Step to add parallel intersection not found");
      return false;
    }
  }

  pub fn insert_alternative_intersection(
    sequence: &mut SequenceProps,
    transition_id: TransitionId,
  ) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| transition_id.0 == x.get_id())
    {
      let entry_transition = sequence.elements.remove(pos);
      let step = sequence.elements.remove(pos);
      let exi_transition = sequence.elements.remove(pos);

      let mut left_branch = SequenceProps::default();
      left_branch.elements.push(entry_transition);
      left_branch.elements.push(step);
      left_branch.elements.push(exi_transition);

      let mut right_branch = SequenceProps::default();
      right_branch
        .elements
        .push(Element::Transition(TransitionProps::default()));
      right_branch
        .elements
        .push(Element::Step(StepProps::default()));
      right_branch
        .elements
        .push(Element::Transition(TransitionProps::default()));

      let new_alternative_intersection = IntersectionProps {
        id: UuidService::new_index(),
        intersection_type: IntersectionType::AlternativeBranches(Branches(vec![
          left_branch,
          right_branch,
        ])),
        on_insert_element_pair_after_intersection: Callback::noop(),
      };

      sequence
        .elements
        .insert(pos, Element::Intersection(new_alternative_intersection));

      return true;
    } else {
      return false;
    }
  }

  pub fn insert_loop_intersection(sequence: &mut SequenceProps, step_id: StepId) -> bool {
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
          return false;
        }
      };

      let new_sequence = SequenceProps {
        elements: vec![step],
        on_insert_element_pair: Callback::noop(),
        on_insert_parallel_intersection: Callback::noop(),
        on_insert_alternative_intersection: Callback::noop(),
        on_insert_loop_intersection: Callback::noop(),
        on_insert_element_pair_after_intersection: Callback::noop(),
      };

      let new_loop_intersection = Element::Intersection(IntersectionProps {
        id: 0,
        intersection_type: IntersectionType::LoopBranches(
          new_sequence,
          TransitionProps::default(),
          exit_transition,
        ),

        on_insert_element_pair_after_intersection: Callback::noop(),
      });

      sequence.elements.insert(pos, new_loop_intersection);
      return true;
    } else {
      Log::error::<Self>("Step to add loop intersection not found");
      return false;
    }
  }

  pub fn append_element_pair_to_intersection(sequence: &mut SequenceProps) -> bool {
    let new_step = Element::Step(StepProps::default());
    let new_transition = Element::Transition(TransitionProps::default());
    if let Some(last_element) = sequence.elements.last() {
      match last_element {
        Element::Step(_) => {
          sequence.elements.push(new_transition);
          sequence.elements.push(new_step);
          return true;
        }
        Element::Transition(_) => {
          sequence.elements.push(new_step);
          sequence.elements.push(new_transition);
          return true;
        }
        _ => {
          todo!("Handle this case");
        }
      }
    } else {
      Log::error::<Self>("No elements to append to");
      return false;
    }
  }
}

impl Component for Sequence {
  type Message = ();
  type Properties = SequenceProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Sequence {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="sequence__container">
        {for ctx.props().elements.iter().enumerate().map(|(index, item)| {
          match item {
            Element::Step(step_props) => {
              let step_id = StepId(step_props.id.clone());
              let mut buttons = Vec::<NetButtonProps>::with_capacity(2);
              buttons.push(NetButtonProps {
                direction: Some(NetButtonDirection::East),
                button_text: "P".to_string(),
                on_click: ctx
                  .props()
                  .on_insert_parallel_intersection
                  .reform(move |_| step_id),
              });

              if LoopIntersectionBehaviour::should_be_viewed(ctx.props(), step_props.id.clone()) {
                buttons.push(NetButtonProps {
                  direction: Some(NetButtonDirection::East),
                  button_text: "L".to_string(),
                  on_click: ctx.props().on_insert_loop_intersection.reform(move |_| step_id),
                });
            }
              html! {
                <Step
                  key={index.clone()}
                  id={step_props.id.clone()}
                  action_name={step_props.action_name.clone()}
                  buttons={buttons}
                  on_insert_parallel_intersection={ctx.props().on_insert_parallel_intersection.clone()}
                  on_insert_loop_intersection={ctx.props().on_insert_loop_intersection.clone()}/>
            }},
            Element::Transition(transition_props) => {
              let id = transition_props.id.clone();

              let mut buttons = Vec::<NetButtonProps>::with_capacity(2);
              buttons.push(NetButtonProps {
                direction: Some(NetButtonDirection::South),
                button_text: "S".to_string(),
                on_click: ctx.props().on_insert_element_pair.reform(move |_| TransitionId(id)),
              });
              if AlternativeIntersectionBehaviour::should_be_viewed(ctx.props(), id) {
                buttons.push(NetButtonProps {
                  direction: Some(NetButtonDirection::NorthEast),
                  button_text: "A".to_string(),
                  on_click: ctx.props().on_insert_alternative_intersection.reform(move |_| TransitionId(id)),
                });
              }
              html! {
                <Transition
                  transitions={transition_props.transitions.clone()}
                  id={transition_props.id.clone()}
                  buttons={buttons}/>
              }
            },
            Element::Intersection(intersection_props) => html! {
              <Intersection
                intersection_type={intersection_props.intersection_type.clone()}
                id={intersection_props.id.clone()}
                on_insert_element_pair_after_intersection={
                  ctx
                  .props()
                  .on_insert_element_pair_after_intersection
                  .reform(|intersection_id|(intersection_id))}/>
            },
          }
        })}
      </div>
    }
  }
}
