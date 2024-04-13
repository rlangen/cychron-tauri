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
    fn new_step(id: usize, step_props: step::StepProps) -> Self {
        Element::Step(step_props)
    }
    fn new_transition(id: usize, transition_props: transition::TransitionProps) -> Self {
        Element::Transition(transition_props)
    }
    fn new_intersection(id: usize, intersection_props: intersection::IntersectionProps) -> Self {
        Element::Intersection(intersection_props)
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
    AddStep(String), // Step id
}

pub struct Sequence {
    petri_net_service: &'static Lazy<Mutex<PetriNetService>>,
}

impl Component for Sequence {
    type Message = SequenceMsg;
    type Properties = SequenceProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Sequence {
            petri_net_service: PetriNetService::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            SequenceMsg::AddStep(step_id) => {
                if let Some(pos) = _ctx.props().elements.iter().position(|x| {
                    if let Element::Transition(transition_props) = x {
                        &transition_props.id == &step_id
                    } else {
                        return false;
                    } // Not a transition
                }) {
                    let petri_net_service = self.petri_net_service.lock().unwrap();
                    let step_id = petri_net_service.new_index();
                    let transition_id = petri_net_service.new_index();
                    std::mem::drop(petri_net_service);

                    let new_step = Element::new_step(
                        step_id,
                        step::StepProps {
                            index: step_id,
                            action_name: "".to_string(),
                        },
                    );
                    _ctx.props().elements.insert(pos + 1, new_step);

                    let new_transition = Element::new_transition(
                        transition_id,
                        transition::TransitionProps {
                            transitions: vec![],
                        },
                    );
                    _ctx.props().elements.insert(pos + 1, new_transition);

                    return true;
                }
                todo!("ERROR: Transition not found")
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
                { for ctx.props().elements.iter().enumerate().map(|(index, item)| {
                    match item {
                        Element::Step(step_props) => html! {
                            <Step
                                key={index.clone()}
                                index={step_props.index.clone()}
                                action_name={step_props.action_name.clone()
                            }/>
                        },
                        Element::Transition(transition_props) => html! {
                            <Transition
                                transitions={transition_props.transitions.clone()}
                            />
                        },
                        Element::Intersection(intersection_props) => html! {
                            <Intersection
                                branches={intersection_props.branches.clone()}
                                intersection_type={intersection_props.intersection_type.clone()}
                                id={intersection_props.id.clone()}
                            />
                        },
                    }
                }) }
            </div>
        }
    }
}
