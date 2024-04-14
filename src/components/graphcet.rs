use yew::prelude::*;

use crate::services::{logging_service::Log, uuid_service::UuidService};

mod sequence;
use sequence::{Element, Sequence, SequenceProps};

use self::sequence::transition::TransitionProps;

pub struct Graphcet {
    sequence: SequenceProps,
}
impl Graphcet {}

impl Component for Graphcet {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let sequence_props = SequenceProps {
            elements: vec![
                Element::Step(sequence::step::StepProps {
                    id: UuidService::new_index(),
                    action_name: "HorizontalCylPaP := 1".to_string(),
                }),
                Element::Transition(sequence::transition::TransitionProps {
                    transitions: "X0".to_string(),
                    id: UuidService::new_index(),
                    on_add_step: Callback::from(|_| ()),
                }),
                Element::Step(sequence::step::StepProps {
                    id: UuidService::new_index(),
                    action_name: "VerticalCylPaP := 1".to_string(),
                }),
                Element::Transition(sequence::transition::TransitionProps {
                    transitions: "X1".to_string(),
                    id: UuidService::new_index(),
                    on_add_step: Callback::from(|_| ()),
                }),
                Element::Intersection(sequence::intersection::IntersectionProps {
                    id: UuidService::new_index(),
                    intersection_type: sequence::intersection::IntersectionType::ParallelBranches(
                        TransitionProps::default(),
                    ),
                    branches: vec![
                        SequenceProps {
                            elements: vec![Element::Step(sequence::step::StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "VerticalCylPaP := 0".to_string(),
                                }),
                            ],
                        },
                        SequenceProps {
                            elements: vec![Element::Step(sequence::step::StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                        },
                    ],
                }),
                Element::Intersection(sequence::intersection::IntersectionProps {
                    id: UuidService::new_index(),
                    intersection_type:
                        sequence::intersection::IntersectionType::AlternativeBranches,
                    branches: vec![
                        SequenceProps {
                            elements: vec![
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "VerticalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    id: UuidService::new_index(),
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                    id: UuidService::new_index(),
                                    on_add_step: Callback::from(|_| ()),
                                }),
                            ],
                        },
                    ],
                }),
                Element::Intersection(sequence::intersection::IntersectionProps {
                    id: UuidService::new_index(),
                    intersection_type: sequence::intersection::IntersectionType::LoopBranches(
                        TransitionProps::default(),
                        TransitionProps::default(),
                    ),
                    branches: vec![SequenceProps {
                        elements: vec![
                            Element::Step(sequence::step::StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            }),
                            Element::Transition(sequence::transition::TransitionProps {
                                transitions: "X2".to_string(),
                                id: UuidService::new_index(),
                                on_add_step: Callback::from(|_| ()),
                            }),
                            Element::Step(sequence::step::StepProps {
                                id: UuidService::new_index(),
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            }),
                        ],
                    }],
                }),
                Element::Step(sequence::step::StepProps {
                    id: UuidService::new_index(),
                    action_name: "HorizontalCylPaP := 0".to_string(),
                }),
            ],
        };
        Self {
            sequence: sequence_props,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Log::info::<Graphcet>("Rendering Graphcet");
        html! {
            <div class="parent">
                    <Sequence elements={self.sequence.clone()}/>
            </div>
        }
    }
}
