use yew::prelude::*;

use crate::services::{logging_service::Log, uuid_service::UuidService};

mod sequence;
use sequence::{
  element::{
    intersection::{IntersectionProps, IntersectionType, TransitionId},
    step::StepProps,
    transition::TransitionProps,
    Element,
  },
  Sequence, SequenceProps,
};

use self::sequence::element::{intersection::parallel_intersection::ParallelIntersection, StepId};

pub enum GraphcetMsg {
  AddStepAndTransition(TransitionId),
  AddParallelIntersection(StepId),
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
          on_add_parallel_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X0".to_string(),
          id: UuidService::new_index(),
          on_add_step: Callback::from(|_| ()),
          on_add_parallel_intersection: Callback::noop(),
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "2VerticalCylPaP := 1".to_string(),
          on_add_parallel_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X1".to_string(),
          id: UuidService::new_index(),
          on_add_step: Callback::from(|_| ()),
          on_add_parallel_intersection: Callback::noop(),
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::ParallelBranches(TransitionProps::default()),
          branches: vec![
            SequenceProps {
              elements: vec![Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "3HorizontalCylPaP := 0".to_string(),
                on_add_parallel_intersection: Callback::noop(),
              })],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "4HorizontalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "5VerticalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
              ],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "6HorizontalCylPaP := 0".to_string(),
                on_add_parallel_intersection: Callback::noop(),
              })],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
          ],
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
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "7HorizontalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
              ],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "8HorizontalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "9VerticalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
              ],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
            SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "HorizontalCylPaP := 0".to_string(),
                  on_add_parallel_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  on_add_step: Callback::from(|_| ()),
                  on_add_parallel_intersection: Callback::noop(),
                }),
              ],
              on_add_step_and_transition: Callback::from(|_| ()),
              on_add_parallel_intersection: Callback::noop(),
            },
          ],
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
                on_add_parallel_intersection: Callback::noop(),
              }),
              Element::Transition(TransitionProps {
                transitions: "X2".to_string(),
                id: UuidService::new_index(),
                on_add_step: Callback::from(|_| ()),
                on_add_parallel_intersection: Callback::noop(),
              }),
              Element::Step(StepProps {
                id: UuidService::new_index(),
                action_name: "HorizontalCylPaP := 0".to_string(),
                on_add_parallel_intersection: Callback::noop(),
              }),
            ],
            on_add_step_and_transition: Callback::from(|_| ()),
            on_add_parallel_intersection: Callback::noop(),
          }],
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "HorizontalCylPaP := 0".to_string(),
          on_add_parallel_intersection: Callback::noop(),
        }),
      ],
      on_add_step_and_transition: Callback::from(|_| ()),
      on_add_parallel_intersection: Callback::noop(),
    };
    Self {
      sequence: sequence_props,
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <Sequence
        elements={self.sequence.clone()}
        on_add_step_and_transition={
          _ctx
          .link()
          .callback(move |transition_id| GraphcetMsg::AddStepAndTransition(transition_id))
        }
        on_add_parallel_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::AddParallelIntersection(data))
        }/>
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      GraphcetMsg::AddStepAndTransition(transition_id) => {
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
            on_add_parallel_intersection: Callback::noop(),
          });
          self.sequence.elements.insert(pos + 1, new_element);

          let id = UuidService::new_index();
          let new_transition = Element::Transition(TransitionProps {
            id,
            transitions: "".to_string(),
            on_add_step: Callback::noop(),
            on_add_parallel_intersection: Callback::noop(),
          });
          self.sequence.elements.insert(pos + 2, new_transition);
          return true;
        } else {
          Log::error::<Self>("Transition to add step and transition not found");
          return false;
        }
      }

      GraphcetMsg::AddParallelIntersection(step_id) => {
        ParallelIntersection::add(&mut self.sequence, step_id)
      }
    }
  }
}
