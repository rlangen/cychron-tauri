use yew::prelude::*;

use crate::{
  components::graphcet::sequence::{
    element::{
      intersection::{BranchIndex, TransitionId},
      step::StepProps,
      transition::{Transition, TransitionProps},
      Element, StepId,
    },
    hover_control::HoverControl,
    Sequence, SequenceProps,
  },
  services::uuid_service::UuidService,
};

use super::{IntersectionProps, IntersectionType};

#[derive(Clone, PartialEq, Properties, Debug)]
pub(crate) struct ParallelIntersecionProps {
  pub exit_transition: TransitionProps,
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_append_transition_and_step: Callback<BranchIndex>,
  pub on_add_step_and_transition: Callback<(BranchIndex, TransitionId)>,
  pub on_add_parallel_intersection: Callback<(BranchIndex, StepId)>,
}

impl Default for ParallelIntersecionProps {
  fn default() -> Self {
    Self {
      exit_transition: TransitionProps::default(),
      id: UuidService::new_index(),
      branches: vec![SequenceProps {
        elements: vec![
          Element::Step(Default::default()),
          Element::Transition(Default::default()),
        ],
        on_add_step_and_transition: Callback::noop(),
        on_add_parallel_intersection: Callback::noop(),
      }],
      on_append_transition_and_step: Callback::noop(),
      on_add_step_and_transition: Callback::noop(),
      on_add_parallel_intersection: Callback::noop(),
    }
  }
}

pub(crate) struct ParallelIntersection;
impl ParallelIntersection {
  pub fn add(
    sequence: &mut SequenceProps,
    step_id: StepId,
  ) -> Result<bool, ParallelIntersectionAddErr> {
    if sequence.elements.len() < 2 {
      return Err(ParallelIntersectionAddErr::SequenceTooShort);
    }

    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| step_id.0 == x.get_id())
    {
      let triggering_step = sequence.elements.remove(pos);
      let triggering_transition;
      match sequence.elements[pos] {
        Element::Transition(_) => match sequence.elements.remove(pos) {
          Element::Transition(transition_props) => {
            triggering_transition = transition_props;
          }
          _ => {
            triggering_transition = TransitionProps::default();
          }
        },
        _ => {
          triggering_transition = TransitionProps::default();
        }
      }

      let new_branch = SequenceProps {
        elements: vec![triggering_step],
        on_add_step_and_transition: Callback::noop(),
        on_add_parallel_intersection: Callback::noop(),
      };

      let new_parallel_intersection = IntersectionProps {
        id: UuidService::new_index(),
        intersection_type: IntersectionType::ParallelBranches(triggering_transition),
        branches: vec![
          new_branch,
          SequenceProps {
            elements: vec![Element::Step(StepProps::default())],
            on_add_step_and_transition: Callback::noop(),
            on_add_parallel_intersection: Callback::noop(),
          },
        ],
      };

      sequence
        .elements
        .insert(pos, Element::Intersection(new_parallel_intersection));
      Ok(true)
    } else {
      Err(ParallelIntersectionAddErr::StepNotFound)
    }
  }
}
pub enum ParallelIntersectionAddErr {
  StepNotFound,
  SequenceTooShort,
}

impl Component for ParallelIntersection {
  type Message = ();
  type Properties = ParallelIntersecionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {<>
      <div class="intersection__parallel-branch-seperation-line"/>
      <div class="intersection__grid-container" key={ctx.props().id.to_string()+"_grid-container"}>
        {for ctx.props().branches.iter().enumerate().map(|(index, item)| {
          html! {
            <div class="intersection__grid-item">
              <div class="intersection__content-wrapper">
                <div class="path__short path__short--margin-left"/>
                  <Sequence
                    elements={item.elements.clone()}
                    on_add_step_and_transition={
                      ctx
                      .props().on_add_step_and_transition
                      .reform(move |transition_id| (BranchIndex(index), transition_id))}
                    on_add_parallel_intersection={
                      ctx
                      .props()
                      .on_add_parallel_intersection
                      .reform(move |step_id| (BranchIndex(index), step_id))
                    }/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="intersection__hover-container">
                <div class="path__short path__short--margin-left"/>
                <HoverControl
                  on_add_step={ctx.props().on_append_transition_and_step.reform(move |_| BranchIndex(index))}
                  on_add_parallel_intersection={Callback::noop()}
                  id={UuidService::new_index()}/>
              </div>
            </div>
          }
        })}
      </div>
      <div class="intersection__parallel-branch-seperation-line"/>
      <Transition
        transitions={ctx.props().exit_transition.transitions.clone()}
        id={ctx.props().exit_transition.id.clone()}
        on_add_step={ctx.props().exit_transition.on_add_step.clone()}
        on_add_parallel_intersection={ctx.props().exit_transition.on_add_parallel_intersection.clone()}/>
    </>}
  }
}
