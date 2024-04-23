use yew::{html::IntoPropValue, prelude::*};

pub mod step;
use step::Step;

pub mod transition;
use transition::Transition;

pub mod intersection;
use intersection::Intersection;

mod hover_control;

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

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct SequenceProps {
    pub elements: Vec<Element>,
    /// <u128> is the id of the transition
    pub on_add_step_and_transition: Callback<u128>,
}
impl IntoPropValue<Vec<Element>> for SequenceProps {
    fn into_prop_value(self) -> Vec<Element> {
        self.elements
    }
}

pub struct Sequence {}

impl Component for Sequence {
    type Message = ();
    type Properties = SequenceProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Sequence {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
            style="
                    display: flex; 
                    flex-direction: column; 
                    align-items: left;
                    margin-left: 0px;
            ">
                { for ctx.props().elements.iter().enumerate().map(|(index, item)| {
                    match item {
                        Element::Step(step_props) => html! {
                            <Step
                                key={index.clone()}
                                id={step_props.id.clone()}
                                action_name={step_props.action_name.clone()
                            }/>
                        },
                        Element::Transition(transition_props) => {
                            let id = transition_props.id.clone();
                            html! {
                                <Transition
                                    transitions={transition_props.transitions.clone()}
                                    id={transition_props.id.clone()}
                                    on_add_step={ctx.props().on_add_step_and_transition.reform(move |_| id)}
                                />
                            }
                        },
                        Element::Intersection(intersection_props) => html! {
                            <Intersection
                                branches={intersection_props.branches.clone()}
                                intersection_type={intersection_props.intersection_type.clone()}
                                id={intersection_props.id.clone()}
                            />
                        },
                    }
                })
                }
            </div>
        }
    }
}
