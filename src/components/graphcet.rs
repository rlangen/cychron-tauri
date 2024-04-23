use yew::prelude::*;

use crate::services::uuid_service::UuidService;

mod sequence;
use sequence::{
    element::{
        intersection::{IntersectionProps, IntersectionType},
        step::StepProps,
        transition::TransitionProps,
        Element,
    },
    Sequence, SequenceProps,
};

pub enum GraphcetMsg {
    /// (Transition id)
    AddStepAndTransition(u128),
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
                    action_name: "HorizontalCylPaP := 1".to_string(),
                }),
                Element::Transition(TransitionProps {
                    transitions: "X0".to_string(),
                    id: UuidService::new_index(),
                    on_add_step: Callback::from(|_| ()),
                }),
                Element::Step(StepProps {
                    id: UuidService::new_index(),
                    action_name: "VerticalCylPaP := 1".to_string(),
                }),
                Element::Transition(TransitionProps {
                    transitions: "X1".to_string(),
                    id: UuidService::new_index(),
                    on_add_step: Callback::from(|_| ()),
                }),
                Element::Intersection(IntersectionProps {
                    id: UuidService::new_index(),
                    intersection_type: IntersectionType::ParallelBranches(
                        TransitionProps::default(),
                    ),
                    branches: vec![
                        SequenceProps {
                            elements: vec![Element::Step(StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                            on_add_step_and_transition: Callback::from(|_| ()),
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "VerticalCylPaP := 0".to_string(),
                                }),
                            ],
                            on_add_step_and_transition: Callback::from(|_| ()),
                        },
                        SequenceProps {
                            elements: vec![Element::Step(StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                            on_add_step_and_transition: Callback::from(|_| ()),
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
                                }),
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                            on_add_step_and_transition: Callback::from(|_| ()),
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "VerticalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                            on_add_step_and_transition: Callback::from(|_| ()),
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                            on_add_step_and_transition: Callback::from(|_| ()),
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
                            }),
                            Element::Transition(TransitionProps {
                                transitions: "X2".to_string(),
                                id: UuidService::new_index(),
                                on_add_step: Callback::from(|_| ()),
                            }),
                            Element::Step(StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            }),
                        ],
                        on_add_step_and_transition: Callback::from(|_| ()),
                    }],
                }),
                Element::Step(StepProps {
                    id: UuidService::new_index(),
                    action_name: "HorizontalCylPaP := 0".to_string(),
                }),
            ],
            on_add_step_and_transition: Callback::from(|_| ()),
        };
        Self {
            sequence: sequence_props,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="parent">
                    <Sequence elements={self.sequence.clone()}
                    on_add_step_and_transition={_ctx.link().callback(move |transition_id| GraphcetMsg::AddStepAndTransition(transition_id))}/>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GraphcetMsg::AddStepAndTransition(transition_id) => {
                if let Some(pos) = self
                    .sequence
                    .elements
                    .iter()
                    .position(|x| transition_id == x.get_id())
                {
                    let id = UuidService::new_index();
                    let new_element = Element::Step(StepProps {
                        id,
                        action_name: "".to_string(),
                    });
                    self.sequence.elements.insert(pos + 1, new_element);

                    let id = UuidService::new_index();
                    let new_transition = Element::Transition(TransitionProps {
                        id,
                        transitions: "".to_string(),
                        on_add_step: ctx
                            .link()
                            .callback(move |_| GraphcetMsg::AddStepAndTransition(id)),
                    });
                    self.sequence.elements.insert(pos + 2, new_transition);
                }
                return true;
            }
        }
    }
}
