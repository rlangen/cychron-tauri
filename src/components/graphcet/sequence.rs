use yew::{html::IntoPropValue, prelude::*};

pub mod step;
use step::Step;

pub mod transition;
use transition::Transition;

pub mod intersection;
use intersection::Intersection;

use crate::services::{logging_service::Log, uuid_service::UuidService};

mod hover_control;

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Step(step::StepProps),
    Transition(transition::TransitionProps),
    Intersection(intersection::IntersectionProps),
}
impl Element {
    fn get_id(&self) -> u128 {
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
}
impl IntoPropValue<Vec<Element>> for SequenceProps {
    fn into_prop_value(self) -> Vec<Element> {
        self.elements
    }
}

pub enum SequenceMsg {
    AddStep(u128), // Step id
}

pub struct Sequence {
    pub elements: Vec<Element>,
}

impl Component for Sequence {
    type Message = SequenceMsg;
    type Properties = SequenceProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Sequence {
            elements: _ctx.props().elements.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            SequenceMsg::AddStep(transition_id) => {
                if let Some(pos) = _ctx
                    .props()
                    .elements
                    .iter()
                    .position(|x| transition_id == x.get_id())
                {
                    let id = UuidService::new_index();
                    let new_step = Element::Step(step::StepProps {
                        id,
                        action_name: "".to_string(),
                    });
                    self.elements.insert(pos + 1, new_step);

                    let id = UuidService::new_index();
                    let new_transition = Element::Transition(transition::TransitionProps {
                        id,
                        transitions: "".to_string(),
                        on_add_step: _ctx.link().callback(move |_| SequenceMsg::AddStep(id)),
                    });
                    self.elements.insert(pos + 2, new_transition);

                    return true;
                }
                Log::error::<Self>("Failed to add step. Could't find transition to append to.");
                return false;
            }
        }
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
                { for self.elements.iter().enumerate().map(|(index, item)| {
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
                                    on_add_step={ctx.link().callback(move |_| SequenceMsg::AddStep(id))}
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
