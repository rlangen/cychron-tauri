use yew::prelude::*;

mod sequence;
use sequence::{Element, Sequence, SequenceProps};

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
                    index: 0,
                    action_name: "HorizontalCylPaP := 1".to_string(),
                }),
                Element::Transition(sequence::transition::TransitionProps {
                    transitions: "X0".to_string(),
                }),
                Element::Step(sequence::step::StepProps {
                    index: 1,
                    action_name: "VerticalCylPaP := 1".to_string(),
                }),
                Element::Transition(sequence::transition::TransitionProps {
                    transitions: "X1".to_string(),
                }),
                Element::Intersection(sequence::intersection::IntersectionProps {
                    intersection_type: sequence::intersection::IntersectionType::ParallelBranches,
                    branches: vec![
                        SequenceProps {
                            elements: vec![Element::Step(sequence::step::StepProps {
                                index: 4,
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                        },
                        SequenceProps {
                            elements: vec![
                                Element::Step(sequence::step::StepProps {
                                    index: 2,
                                    action_name: "HorizontalCylPaP := 0".to_string(),
                                }),
                                Element::Transition(sequence::transition::TransitionProps {
                                    transitions: "X2".to_string(),
                                }),
                                Element::Step(sequence::step::StepProps {
                                    index: 3,
                                    action_name: "VerticalCylPaP := 0".to_string(),
                                }),
                            ],
                        },
                        SequenceProps {
                            elements: vec![Element::Step(sequence::step::StepProps {
                                index: 4,
                                action_name: "HorizontalCylPaP := 0".to_string(),
                            })],
                        },
                    ],
                }),
            ],
        };
        Self {
            sequence: sequence_props,
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="parent">
                <Sequence elements={self.sequence.clone()}/>
            </div>
        }
    }
}
