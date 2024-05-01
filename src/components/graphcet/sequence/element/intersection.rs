use yew::prelude::*;

use self::parallel_intersection::ParallelIntersectionAddErr;

use super::{transition::TransitionProps, StepId};
use crate::{
  components::graphcet::sequence::{
    element::{step::StepProps, Element},
    Sequence, SequenceProps,
  },
  services::{logging_service::Log, uuid_service::UuidService},
};

pub(crate) mod parallel_intersection;
use parallel_intersection::ParallelIntersection;

pub(crate) mod alternative_intersection;
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

  pub on_attach_element_pair_to_intersection: Callback<IntersectionId>,
}
impl Default for IntersectionType {
  fn default() -> Self {
    IntersectionType::ParallelBranches(TransitionProps::default())
  }
}

pub enum IntersectionMsg {
  AddBranch((BranchIndex, AddToLeft)),

  PrependElementPair(BranchIndex),
  InsertElementPair((BranchIndex, TransitionId)),
  AppendElementPair(BranchIndex),

  AttachElementPairToIntersection((BranchIndex, IntersectionId)),

  InsertParallelIntersection((BranchIndex, StepId)),
  InsertAlternativeIntersection((BranchIndex, TransitionId)),
}
#[derive(Copy, Clone)]
pub struct BranchIndex(pub usize);
#[derive(Copy, Clone)]
pub struct TransitionId(pub u128);
#[derive(Copy, Clone)]
pub struct AddToLeft(pub bool);
#[derive(Copy, Clone)]
pub struct IntersectionId(pub u128);

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
    let id = ctx.props().id;
    html! {
      <div class= "intersection__top-level-container">
      {match &ctx.props().intersection_type {
        IntersectionType::ParallelBranches(exit_transition) => html! {
          <ParallelIntersection
            branches={self.branches.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}
            on_insert_element_pair={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertElementPair(data))}
            on_prepend_element_pair={
              ctx
              .link()
              .callback(|branch_index|IntersectionMsg::PrependElementPair(branch_index))}
            on_append_element_pair={
              ctx
              .link()
              .callback(|branch_index|IntersectionMsg::AppendElementPair(branch_index))}
            on_insert_parallel_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertParallelIntersection(data))}
            on_add_branch={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddBranch(data))}
            on_attach_element_pair_to_intersection={
              ctx
              .props()
              .on_attach_element_pair_to_intersection
              .clone()
              .reform(move |_| IntersectionId(id))}
            on_pass_attach_element_pair_to_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AttachElementPairToIntersection(data))
            }
            on_insert_alternative_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertAlternativeIntersection(data))
            }/>
        },
        IntersectionType::AlternativeBranches => html! {
          <AlternativeIntersection
            branches={self.branches.clone()}
            id={UuidService::new_index()}
            on_add_branch={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AddBranch(data))}
            on_prepend_element_pair={
              ctx
              .link()
              .callback(|branch_index|IntersectionMsg::PrependElementPair(branch_index))}
            on_append_element_pair={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertElementPair(data))}
            on_insert_parallel_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertParallelIntersection(data))
            }
            on_insert_alternative_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertAlternativeIntersection(data))
            }
            on_pass_attach_element_pair_to_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AttachElementPairToIntersection(data))}/>
        },
        IntersectionType::LoopBranches(continue_transition, exit_transition) => html! {
          <LoopIntersection
            branches={self.branches.clone()}
            continue_transition={continue_transition.clone()}
            exit_transition={exit_transition.clone()}
            id={UuidService::new_index()}
            on_prepend_element_pair={
              ctx
              .link()
              .callback(|branch_index|IntersectionMsg::PrependElementPair(branch_index))}
            on_append_element_pair={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertElementPair(data))}
            on_insert_parallel_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertParallelIntersection(data))}
            on_insert_alternative_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::InsertAlternativeIntersection(data))
            }
            on_pass_attach_element_pair_to_intersection={
              ctx
              .link()
              .callback(|data|IntersectionMsg::AttachElementPairToIntersection(data))}/>
        },
      }}
    </div>}
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      IntersectionMsg::InsertElementPair((branch_index, transition_id)) => {
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
            on_insert_parallel_intersection: Callback::noop(),
          });
          self.branches[branch_index]
            .elements
            .insert(pos + 1, new_element);

          let id = UuidService::new_index();
          let new_transition = Element::Transition(TransitionProps {
            id,
            transitions: "".to_string(),
            buttons: vec![],
          });
          self.branches[branch_index]
            .elements
            .insert(pos + 2, new_transition);

          return true;
        }
        Log::error::<Self>("Failed to add step. Could't find transition to append to.");
        return false;
      }

      IntersectionMsg::AppendElementPair(branch_index) => {
        let branch_index_number = branch_index.0;

        let id = UuidService::new_index();
        let new_transition = Element::Transition(TransitionProps {
          id,
          transitions: "".to_string(),
          buttons: vec![],
        });
        self.branches[branch_index_number]
          .elements
          .push(new_transition);

        let id = UuidService::new_index();
        let new_step = Element::Step(StepProps {
          id,
          action_name: "".to_string(),
          on_insert_parallel_intersection: Callback::noop(),
        });
        self.branches[branch_index_number].elements.push(new_step);

        return true;
      }
      IntersectionMsg::InsertParallelIntersection((branch_index, step_id)) => {
        match ParallelIntersection::add(&mut self.branches[branch_index.0], step_id) {
          Ok(needs_update) => {
            return needs_update;
          }
          Err(err) => match err {
            ParallelIntersectionAddErr::SequenceTooShort => {
              self.branches[branch_index.0]
                .elements
                .push(Element::Transition(TransitionProps::default()));

              self.branches[branch_index.0]
                .elements
                .push(Element::Step(StepProps::default()));

              match ParallelIntersection::add(&mut self.branches[branch_index.0], step_id) {
                Ok(needs_update) => {
                  return needs_update;
                }
                Err(_) => {
                  Log::error::<Self>("Failed to add parallel intersection.");
                  return false;
                }
              }
            }
            ParallelIntersectionAddErr::StepNotFound => {
              Log::error::<Self>("Failed to add parallel intersection. Step not found.");
              false
            }
          },
        }
      }

      IntersectionMsg::AddBranch((branch_index, add_to_left)) => {
        let new_sequence;
        match ctx.props().intersection_type {
          IntersectionType::ParallelBranches(_) => {
            new_sequence = SequenceProps {
              elements: vec![Element::Step(StepProps::default())],
              on_insert_element_pair: Callback::noop(),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            };
          }
          IntersectionType::AlternativeBranches => {
            new_sequence = SequenceProps {
              elements: vec![
                Element::Transition(TransitionProps::default()),
                Element::Step(StepProps::default()),
                Element::Transition(TransitionProps::default()),
              ],
              on_insert_element_pair: Callback::noop(),
              on_insert_parallel_intersection: Callback::noop(),
              on_insert_alternative_intersection: Callback::noop(),
              on_attach_element_pair_to_intersection: Callback::noop(),
            };
          }
          IntersectionType::LoopBranches(_, _) => {
            Log::error::<Self>("Failed to add branch. Loop branches not supported.");
            return false;
          }
        }

        if add_to_left.0 {
          self.branches.insert(branch_index.0, new_sequence);
        } else {
          self.branches.insert(branch_index.0 + 1, new_sequence);
        }
        true
      }

      IntersectionMsg::PrependElementPair(branch_index) => {
        match ctx.props().intersection_type {
          IntersectionType::ParallelBranches(_) | IntersectionType::LoopBranches(_, _) => {
            self.branches[branch_index.0]
              .elements
              .insert(0, Element::Step(StepProps::default()));
            if self.branches[branch_index.0].elements.len() > 1 {
              self.branches[branch_index.0]
                .elements
                .insert(1, Element::Transition(TransitionProps::default()));
            }
          }
          IntersectionType::AlternativeBranches => {
            self.branches[branch_index.0]
              .elements
              .insert(0, Element::Transition(TransitionProps::default()));
            self.branches[branch_index.0]
              .elements
              .insert(1, Element::Step(StepProps::default()));
          }
        }
        true
      }
      IntersectionMsg::AttachElementPairToIntersection((branch_index, intersection_id)) => {
        Sequence::attach_element_pair_to_intersection(
          &mut self.branches[branch_index.0].elements,
          intersection_id,
        )
      }
      IntersectionMsg::InsertAlternativeIntersection((branch_index, transition_id)) => {
        let should_rerender =
          AlternativeIntersection::add(&mut self.branches[branch_index.0], transition_id);
        if !should_rerender.0 {
          Log::error::<Self>("Transition to add alternative intersection not found");
        }
        return should_rerender.0;
      }
    }
  }
}
