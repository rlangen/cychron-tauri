use yew::{html::IntoPropValue, prelude::*};

pub mod element;
use element::{
  intersection::{Intersection, TransitionId},
  step::Step,
  transition::Transition,
  Element,
};

mod hover_control;

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct SequenceProps {
  pub elements: Vec<Element>,
  /// <u128> is the id of the transition
  pub on_add_step_and_transition: Callback<TransitionId>,
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
      <div class="sequence__container">
        {for ctx.props().elements.iter().enumerate().map(|(index, item)| {
          match item {
            Element::Step(step_props) => html! {
              <Step
                key={index.clone()}
                id={step_props.id.clone()}
                action_name={step_props.action_name.clone()}/>
            },
            Element::Transition(transition_props) => {
              let id = transition_props.id.clone();
              html! {
                <Transition
                  transitions={transition_props.transitions.clone()}
                  id={transition_props.id.clone()}
                  on_add_step={ctx.props().on_add_step_and_transition.reform(move |_| TransitionId(id))}/>
              }
            },
            Element::Intersection(intersection_props) => html! {
              <Intersection
                branches={intersection_props.branches.clone()}
                intersection_type={intersection_props.intersection_type.clone()}
                id={intersection_props.id.clone()}/>
            },
          }
        })}
      </div>
    }
  }
}
