use yew::prelude::*;

use crate::services::uuid_service::UuidService;

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

use self::sequence::element::{
  intersection::{Branches, IntersectionId},
  StepId,
};

pub enum GraphcetMsg {
  InsertElementPair(TransitionId),
  InsertParallelIntersection(StepId),
  InsertAlternativeIntersection(TransitionId),
  InsertLoopIntersection(StepId),
  InsertElementPairAfterIntersechtion(IntersectionId),
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
          buttons: vec![],
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X0".to_string(),
          id: UuidService::new_index(),
          buttons: vec![],
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "2VerticalCylPaP := 1".to_string(),
          buttons: vec![],
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
        }),
        Element::Transition(TransitionProps {
          transitions: "X1".to_string(),
          id: UuidService::new_index(),
          buttons: vec![],
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::ParallelBranches(
            Branches(vec![
              SequenceProps {
                elements: vec![Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "3HorizontalCylPaP := 0".to_string(),
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
                })],
                on_insert_element_pair: Callback::from(|_| ()),
                on_insert_parallel_intersection: Callback::noop(),
                on_insert_alternative_intersection: Callback::noop(),
                on_insert_loop_intersection: Callback::noop(),
                on_insert_element_pair_after_intersection: Callback::noop(),
              },
              SequenceProps {
                elements: vec![
                  Element::Step(StepProps {
                    id: UuidService::new_index(),
                    action_name: "4HorizontalCylPaP := 0".to_string(),
                    buttons: vec![],
                    on_insert_parallel_intersection: Callback::noop(),
                    on_insert_loop_intersection: Callback::noop(),
                  }),
                  Element::Transition(TransitionProps {
                    transitions: "X2".to_string(),
                    id: UuidService::new_index(),
                    buttons: vec![],
                  }),
                  Element::Step(StepProps {
                    id: UuidService::new_index(),
                    action_name: "5VerticalCylPaP := 0".to_string(),
                    buttons: vec![],
                    on_insert_parallel_intersection: Callback::noop(),
                    on_insert_loop_intersection: Callback::noop(),
                  }),
                ],
                on_insert_element_pair: Callback::from(|_| ()),
                on_insert_parallel_intersection: Callback::noop(),
                on_insert_alternative_intersection: Callback::noop(),
                on_insert_loop_intersection: Callback::noop(),
                on_insert_element_pair_after_intersection: Callback::noop(),
              },
              SequenceProps {
                elements: vec![Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "6HorizontalCylPaP := 0".to_string(),
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
                })],
                on_insert_element_pair: Callback::from(|_| ()),
                on_insert_parallel_intersection: Callback::noop(),
                on_insert_alternative_intersection: Callback::noop(),
                on_insert_loop_intersection: Callback::noop(),
                on_insert_element_pair_after_intersection: Callback::noop(),
              },
            ]),
            TransitionProps::default(),
          ),
          on_insert_element_pair_after_intersection: Callback::noop(),
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::AlternativeBranches(Branches(vec![
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
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
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
              on_insert_loop_intersection: Callback::noop(),
              on_insert_element_pair_after_intersection: Callback::noop(),
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
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "9VerticalCylPaP := 0".to_string(),
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
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
              on_insert_loop_intersection: Callback::noop(),
              on_insert_element_pair_after_intersection: Callback::noop(),
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
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
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
              on_insert_loop_intersection: Callback::noop(),
              on_insert_element_pair_after_intersection: Callback::noop(),
            },
          ])),
          on_insert_element_pair_after_intersection: Callback::noop(),
        }),
        Element::Intersection(IntersectionProps {
          id: UuidService::new_index(),
          intersection_type: IntersectionType::LoopBranches(
            SequenceProps {
              elements: vec![
                Element::Step(StepProps {
                  id: UuidService::new_index(),
                  action_name: "HorizontalCylPaP := 0".to_string(),
                  buttons: vec![],
                  on_insert_parallel_intersection: Callback::noop(),
                  on_insert_loop_intersection: Callback::noop(),
                }),
                Element::Transition(TransitionProps {
                  transitions: "X2".to_string(),
                  id: UuidService::new_index(),
                  buttons: vec![],
                }),
                Element::Step(StepProps::default()),
              ],
              on_insert_element_pair: Callback::from(|_| ()),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_insert_loop_intersection: Callback::noop(),
              on_insert_element_pair_after_intersection: Callback::noop(),
            },
            TransitionProps::default(),
            TransitionProps::default(),
          ),
          on_insert_element_pair_after_intersection: Callback::noop(),
        }),
        Element::Step(StepProps {
          id: UuidService::new_index(),
          action_name: "HorizontalCylPaP := 0".to_string(),
          buttons: vec![],
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
        }),
      ],
      on_insert_element_pair: Callback::from(|_| ()),
      on_insert_parallel_intersection: Callback::noop(),
      on_insert_alternative_intersection: Callback::noop(),
      on_insert_loop_intersection: Callback::noop(),
      on_insert_element_pair_after_intersection: Callback::noop(),
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
        on_insert_element_pair_after_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertElementPairAfterIntersechtion(data))
        }
        on_insert_parallel_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertParallelIntersection(data))
        }
        on_insert_alternative_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertAlternativeIntersection(data))
        }
        on_insert_loop_intersection={
          _ctx
          .link()
          .callback(|data|GraphcetMsg::InsertLoopIntersection(data))
        }/>
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      GraphcetMsg::InsertElementPair(transition_id) => {
        Sequence::insert_element_pair(&mut self.sequence, transition_id)
      }

      GraphcetMsg::InsertParallelIntersection(step_id) => {
        Sequence::insert_parallel_intersection(&mut self.sequence, step_id)
      }

      GraphcetMsg::InsertElementPairAfterIntersechtion(intersection_id) => {
        Sequence::insert_element_pair_after_intersection(&mut self.sequence, intersection_id)
      }
      GraphcetMsg::InsertAlternativeIntersection(transition_id) => {
        Sequence::insert_alternative_intersection(&mut self.sequence, transition_id)
      }

      GraphcetMsg::InsertLoopIntersection(step_id) => {
        Sequence::insert_loop_intersection(&mut self.sequence, step_id)
      }
    }
  }
}
