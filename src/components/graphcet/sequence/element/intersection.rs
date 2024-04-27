use yew::prelude::*;

use super::{transition::TransitionProps, StepId};
use crate::{
  components::graphcet::sequence::{
    element::{step::StepProps, Element},
    SequenceProps,
  },
  services::{logging_service::Log, uuid_service::UuidService},
};

pub(crate) mod parallel_intersection;
use parallel_intersection::ParallelIntersection;

mod alternative_intersection;
use alternative_intersection::AlternativeIntersection;

mod loop_intersection;
use loop_intersection::LoopIntersection;

#[derive(Clone, PartialEq, Debug)]
pub enum IntersectionType {
  ParallelBranches(TransitionProps),
  AlternativeBranches,
  /// First transition exits loop, second continues loop
  LoopBranches(TransitionProps, TransitionProps),
}

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct IntersectionProps {
  pub branches: Vec<SequenceProps>,
  pub intersection_type: IntersectionType,
  pub id: u128,
}
impl Default for IntersectionType {
  fn default() -> Self {
    IntersectionType::ParallelBranches(TransitionProps::default())
  }
}

pub enum IntersectionMsg {
  AddStepAndTransition((BranchIndex, TransitionId)),
  AppendTransitionAndStep(BranchIndex),
  AddParallelIntersection((BranchIndex, StepId)),
}
#[derive(Copy, Clone)]
pub struct BranchIndex(pub usize);
#[derive(Copy, Clone)]
pub struct TransitionId(pub u128);

pub struct Intersection {
  branches: Vec<SequenceProps>,
}

impl Component for Intersection {
  type Message = IntersectionMsg;
  type Properties = IntersectionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      branches: _ctx.props().branches.clone(),
    }
  }
  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class= "intersection__top-level-container">
      {match &ctx.props().intersection_type {
        IntersectionType::ParallelBranches(exit_transition) => html! {
          <ParallelIntersection
            branches={self.branches.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}
            on_add_step_and_transition={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddStepAndTransition(data))}
            on_append_transition_and_step={
              ctx
              .link()
              .callback(|branch_index|IntersectionMsg::AppendTransitionAndStep(branch_index))}
            on_add_parallel_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddParallelIntersection(data))
            }/>
        },
        IntersectionType::AlternativeBranches => html! {
          <AlternativeIntersection
            branches={self.branches.clone()}
            id={UuidService::new_index()}
            on_append_step_and_transition={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddStepAndTransition(data))}
            on_add_parallel_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddParallelIntersection(data))
            }/>
        },
        IntersectionType::LoopBranches(continue_transition, exit_transition) => html! {
          <LoopIntersection
            branches={self.branches.clone()}
            continue_transition={continue_transition.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}
            on_append_step_and_transition={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddStepAndTransition(data))}
              on_add_parallel_intersection={
                ctx
                .link()
                .callback(|data|IntersectionMsg::AddParallelIntersection(data))
              }/>
        },
      }}
    </div>}
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      IntersectionMsg::AddStepAndTransition((branch_index, transition_id)) => {
        let branch_index = branch_index.0;
        let transition_id = transition_id.0;

        if let Some(pos) = ctx.props().branches[branch_index]
          .elements
          .iter()
          .position(|x| transition_id == x.get_id())
        {
          let id = UuidService::new_index();
          let new_element = Element::Step(StepProps {
            id,
            action_name: "".to_string(),
            on_add_parallel_intersection: Callback::noop(),
          });
          self.branches[branch_index]
            .elements
            .insert(pos + 1, new_element);

          let id = UuidService::new_index();
          let new_transition = Element::Transition(TransitionProps {
            id,
            transitions: "".to_string(),
            on_add_step: ctx.link().callback(move |_| {
              IntersectionMsg::AddStepAndTransition((
                BranchIndex(branch_index),
                TransitionId(transition_id),
              ))
            }),
            on_add_parallel_intersection: Callback::noop(),
          });
          self.branches[branch_index]
            .elements
            .insert(pos + 2, new_transition);

          return true;
        }
        Log::error::<Self>("Failed to add step. Could't find transition to append to.");
        return false;
      }

      IntersectionMsg::AppendTransitionAndStep(branch_index) => {
        let branch_index_number = branch_index.0;

        let id = UuidService::new_index();
        let new_transition = Element::Transition(TransitionProps {
          id,
          transitions: "".to_string(),
          on_add_step: ctx.link().callback(move |_| {
            IntersectionMsg::AddStepAndTransition((branch_index, TransitionId(id)))
          }),
          on_add_parallel_intersection: Callback::noop(),
        });
        self.branches[branch_index_number]
          .elements
          .push(new_transition);

        let id = UuidService::new_index();
        let new_step = Element::Step(StepProps {
          id,
          action_name: "".to_string(),
          on_add_parallel_intersection: Callback::noop(),
        });
        self.branches[branch_index_number].elements.push(new_step);

        return true;
      }
      IntersectionMsg::AddParallelIntersection((branch_index, step_id)) => {
        ParallelIntersection::add(&mut self.branches[branch_index.0], step_id)
      }
    }
  }
}
