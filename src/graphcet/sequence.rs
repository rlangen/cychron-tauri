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

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct SequenceProps {
    pub elements: Vec<Element>,
}
impl IntoPropValue<Vec<Element>> for SequenceProps {
    fn into_prop_value(self) -> Vec<Element> {
        // Convert self into a Vec<sequence::Element> here
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
