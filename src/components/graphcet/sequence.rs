use yew::{html::IntoPropValue, prelude::*};

pub mod element;
use element::{
  intersection::{Intersection, IntersectionId, TransitionId},
  step::Step,
  transition::Transition,
  Element,
};

use crate::{
  components::net_button::{NetButtonDirection, NetButtonProps},
  services::logging_service::Log,
};

use self::element::{step::StepProps, transition::TransitionProps, StepId};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct SequenceProps {
  pub elements: Vec<Element>,
  pub on_insert_element_pair: Callback<TransitionId>,
  pub on_insert_parallel_intersection: Callback<StepId>,
  pub on_insert_alternative_intersection: Callback<TransitionId>,
  pub on_attach_element_pair_to_intersection: Callback<IntersectionId>,
}
impl IntoPropValue<Vec<Element>> for SequenceProps {
  fn into_prop_value(self) -> Vec<Element> {
    self.elements
  }
}

pub struct Sequence;
impl Sequence {
  pub fn attach_element_pair_to_intersection(
    elements: &mut Vec<Element>,
    intersection_id: IntersectionId,
  ) -> bool {
    if let Some(pos) = elements
      .iter()
      .position(|x| intersection_id.0 == x.get_id())
    {
      let new_step = Element::Step(StepProps::default());
      elements.insert(pos + 1, new_step);

      let new_transition = Element::Transition(TransitionProps::default());
      elements.insert(pos + 2, new_transition);

      return true;
    } else {
      Log::error::<Self>("Intersection to add step and transition not found");
      return false;
    }
  }
}

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
                action_name={step_props.action_name.clone()}
                on_insert_parallel_intersection={ctx.props().on_insert_parallel_intersection.clone()}/>
            },
            Element::Transition(transition_props) => {
              let id = transition_props.id.clone();

              let mut buttons = Vec::<NetButtonProps>::with_capacity(2);
              buttons.push(NetButtonProps {
                direction: Some(NetButtonDirection::South),
                button_text: "S".to_string(),
                on_click: ctx.props().on_insert_element_pair.reform(move |_| TransitionId(id)),
              });
              // It doesn't make sense to add an alternative intersection before a transition which has no following step or intersection
              if index != ctx.props().elements.len() - 1 {
                buttons.push(NetButtonProps {
                  direction: Some(NetButtonDirection::NorthEast),
                  button_text: "A".to_string(),
                  on_click: ctx.props().on_insert_alternative_intersection.reform(move |_| TransitionId(id)),
                });
              }

              html! {
                <Transition
                  transitions={transition_props.transitions.clone()}
                  id={transition_props.id.clone()}
                  buttons={buttons}/>
              }
            },
            Element::Intersection(intersection_props) => html! {
              <Intersection
                branches={intersection_props.branches.clone()}
                intersection_type={intersection_props.intersection_type.clone()}
                id={intersection_props.id.clone()}
                on_attach_element_pair_to_intersection={
                  ctx
                  .props()
                  .on_attach_element_pair_to_intersection
                  .reform(|intersection_id|(intersection_id))}/>
            },
          }
        })}
      </div>
    }
  }
}
