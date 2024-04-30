use yew::prelude::*;

use crate::services::{logging_service::Log, uuid_service::UuidService};

mod sequence;
use sequence::{
  element::{
    intersection::{
      alternative_intersection::{AlternativeIntersection, AlternativeIntersectionAddError},
      IntersectionProps, IntersectionType, TransitionId,
    },
    step::StepProps,
    transition::TransitionProps,
    Element,
  },
  Sequence, SequenceProps,
};

use self::sequence::element::{
  intersection::{
    parallel_intersection::{ParallelIntersection, ParallelIntersectionAddErr},
    IntersectionId,
  },
  StepId,
};

pub enum GraphcetMsg {
  InsertElementPair(TransitionId),
  InsertParallelIntersection(StepId),
  InsertAlternativeIntersection(TransitionId),
  AttachElementPairToIntersection(IntersectionId),
}

pub struct Graphcet {
  sequence: SequenceProps,
}
impl Graphcet {}

impl Component for Graphcet {
  type Message = GraphcetMsg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let sequence_props = SequenceProps {
      elements: vec![
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "1HorizontalCylPaP := 1".to_string(),
          on_insert_parallel_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X0".to_string(),
          id: UuidService::new_index(),
          buttons: vec![],
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "2VerticalCylPaP := 1".to_string(),
          on_insert_parallel_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X1".to_string(),
          id: UuidService::new_index(),
          buttons: vec![],
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::ParallelBranches(TransitionProps::default()),
          branches: vec![
            SequenceProps {
              elements: vec![Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "3HorizontalCylPaP := 0".to_string(),
                on_insert_parallel_intersection: Callback::noop(),
              })],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "4HorizontalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "5VerticalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
              ],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "6HorizontalCylPaP := 0".to_string(),
                on_insert_parallel_intersection: Callback::noop(),
              })],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
          ],
          on_attach_element_pair_to_intersection: Callback::noop(),
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::AlternativeBranches,
          branches: vec![
            SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "7HorizontalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
              ],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "8HorizontalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "9VerticalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
              ],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "HorizontalCylPaP := 0".to_string(),
                  on_insert_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
              ],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            },
          ],
          on_attach_element_pair_to_intersection: Callback::noop(),
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::LoopBranches(
            TransitionProps::default(),
            TransitionProps::default(),
          ),
          branches: vec![SequenceProps {
            elements: vec![
              Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "HorizontalCylPaP := 0".to_string(),
                on_insert_parallel_intersection: Callback::noop(),
              }),
              Element::Transition(TransitionProps {
                transitions: "X2".to_string(),
                id: UuidService::new_index(),
                buttons: vec![],
              }),
              Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "HorizontalCylPaP := 0".to_string(),
                on_insert_parallel_intersection: Callback::noop(),
              }),
            ],
            on_insert_element_pair: Callback::from(|_| ()),
            on_insert_parallel_intersection: Callback::noop(),
            on_insert_alternative_intersection: Callback::noop(),
            on_attach_element_pair_to_intersection: Callback::noop(),
          }],
          on_attach_element_pair_to_intersection: Callback::noop(),
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "HorizontalCylPaP := 0".to_string(),
          on_insert_parallel_intersection: Callback::noop(),
        }),
      ],
      on_insert_element_pair: Callback::from(|_| ()),
      on_insert_parallel_intersection: Callback::noop(),
      on_insert_alternative_intersection: Callback::noop(),
      on_attach_element_pair_to_intersection: Callback::noop(),
    };
    Self {
      sequence: sequence_props,
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <Sequence
        elements={self.sequence.clone()}
        on_insert_element_pair={
          _ctx
          .link()
          .callback(move |transition_id| GraphcetMsg::InsertElementPair(transition_id))
        }
        on_insert_parallel_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertParallelIntersection(data))
        }
        on_attach_element_pair_to_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::AttachElementPairToIntersection(data))
        }
        on_insert_alternative_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertAlternativeIntersection(data))
        }/>
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      GraphcetMsg::InsertElementPair(transition_id) => {
        if let Some(pos) = self
          .sequence
          .elements
          .iter()
          .position(|x| transition_id.0 == x.get_id())
        {
          let id = UuidService::new_index();
          let new_element = Element::Step(StepProps {
            id,
            action_name: "".to_string(),
            on_insert_parallel_intersection: Callback::noop(),
          });
          self.sequence.elements.insert(pos + 1, new_element);

          let id = UuidService::new_index();
          let new_transition = Element::Transition(TransitionProps {
            id,
            transitions: "".to_string(),
            buttons: vec![],
          });
          self.sequence.elements.insert(pos + 2, new_transition);
          return true;
        } else {
          Log::error::<Self>("Transition to add step and transition not found");
          return false;
        }
      }

      GraphcetMsg::InsertParallelIntersection(step_id) => {
        match ParallelIntersection::add(&mut self.sequence, step_id) {
          Ok(needs_update) => needs_update,
          Err(err) => {
            match err {
              ParallelIntersectionAddErr::StepNotFound => {
                Log::error::<Self>("Failed to add parallel intersection. Step not found");
              }
              ParallelIntersectionAddErr::SequenceTooShort => {
                Log::error::<Self>("Failed to add parallel intersection. Sequence too short");
              }
            }
            return false;
          }
        }
      }

      GraphcetMsg::AttachElementPairToIntersection(intersection_id) => {
        Sequence::attach_element_pair_to_intersection(&mut self.sequence.elements, intersection_id)
      }
      GraphcetMsg::InsertAlternativeIntersection(transition_id) => {
        return match AlternativeIntersection::add(&mut self.sequence, transition_id) {
          Ok(needs_update) => needs_update.0,
          Err(err) => {
            match err {
              AlternativeIntersectionAddError::TransitionNotFound => {
                Log::error::<Self>("Failed to add alternative intersection. Transition not found.");
              }
              AlternativeIntersectionAddError::NoStepAfterTransition => {
                Log::error::<Self>(
                  "Failed to add alternative intersection. No step after transition.",
                );
              }
              AlternativeIntersectionAddError::NoTransitionAtEnd => {
                Log::error::<Self>("Failed to add alternative intersection. No transition at end.");
              }
            }

            false
          }
        }
      }
    }
  }
}
