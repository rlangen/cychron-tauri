use yew::prelude::*;

use crate::components::{
  graphcet::sequence::{
    element::{
      intersection::{AddToLeft, BranchIndex, IntersectionId, TransitionId},
      step::StepProps,
      transition::TransitionProps,
      Element, StepId,
    },
    Sequence, SequenceProps,
  },
  net_button::{NetButtonDirection, NetButtonProps},
  net_user_control::NetUserControl,
};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct AlternativeIntersectionProps {
  pub id: u128,
  pub branches: Vec<SequenceProps>,
}

pub(crate) struct AlternativeIntersection {
  branches: Vec<SequenceProps>,
}

pub enum AlternativeIntersectionMsg {
  // <<<--- AlternativeIntersection operations --->>>
  AddBranch(BranchIndex, AddToLeft),
  PrependElementPair(BranchIndex),
  // <<<--- Sequence operations --->>>
  SeqInsertElementPair(BranchIndex, TransitionId),
  SeqInsertElementPairAfterIntersection(BranchIndex, IntersectionId),
  SeqInsertParallelIntersection(BranchIndex, StepId),
  SeqInsertAlternativeIntersection(BranchIndex, TransitionId),
  SeqInsertLoopIntersection(BranchIndex, StepId),
}

impl Component for AlternativeIntersection {
  type Message = AlternativeIntersectionMsg;

  type Properties = AlternativeIntersectionProps;

  fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
    Self {
      branches: _ctx.props().branches.clone(),
    }
  }

  fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
    html! {<>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
      <div class="intersection__grid-container">
        {for self.branches.iter().enumerate().map(|(index, item)| {
          html! {
            <div class="intersection__grid-item">
              <div class="intersection__content-wrapper">
                <div class="
                  path__short 
                  path__short--margin-left 
                  intersection__entry-menu">
                  <NetUserControl
                    buttons={vec![
                      NetButtonProps {
                        direction: Some(NetButtonDirection::West),
                        button_text: "B".to_string(),
                        on_click: ctx.link().callback(move |_| AlternativeIntersectionMsg::AddBranch(BranchIndex(index), AddToLeft(true))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::South),
                        button_text: "S".to_string(),
                        on_click: ctx.link().callback(move |_| AlternativeIntersectionMsg::PrependElementPair(BranchIndex(index))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::East),
                        button_text: "B".to_string(),
                        on_click: ctx.link().callback(move |_| AlternativeIntersectionMsg::AddBranch(BranchIndex(index), AddToLeft(false))),
                      },
                    ]}/>
                </div>
                <Sequence
                  key={index.clone()}
                  elements={item.elements.clone()}
                  on_insert_element_pair={
                    ctx
                    .link()
                    .callback(move |transition_id| AlternativeIntersectionMsg::SeqInsertElementPair(BranchIndex(index), transition_id))
                  }
                  on_insert_element_pair_after_intersection={
                    ctx
                    .link()
                    .callback(move |intersection_id| AlternativeIntersectionMsg::SeqInsertElementPairAfterIntersection(BranchIndex(index), intersection_id))
                  }
                  on_insert_parallel_intersection={
                    ctx
                    .link()
                    .callback(move |step_id| AlternativeIntersectionMsg::SeqInsertParallelIntersection(BranchIndex(index), step_id))
                  }
                  on_insert_alternative_intersection={
                    ctx
                    .link()
                    .callback(move |transition_id| AlternativeIntersectionMsg::SeqInsertAlternativeIntersection(BranchIndex(index), transition_id))
                  }
                  on_insert_loop_intersection={
                    ctx
                    .link()
                    .callback(move |step_id| AlternativeIntersectionMsg::SeqInsertLoopIntersection(BranchIndex(index), step_id))
                  }/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="path__short path__short--margin-left"/>
            </div>
          }
        })}
      </div>
      <div
        key={ctx.props().id}
        class="intersection__alternative-branch-seperation-line"/>
      <div class="path__short path__short--margin-left"/>
    </>}
  }

  fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      // ------------------------------------------------
      // <<<--- AlternativeIntersection operations --->>>
      // ------------------------------------------------
      AlternativeIntersectionMsg::AddBranch(branch_index, add_to_left) => {
        let new_sequence = SequenceProps {
          elements: vec![
            Element::Transition(TransitionProps::default()),
            Element::Step(StepProps::default()),
            Element::Transition(TransitionProps::default()),
          ],
          on_insert_element_pair: Callback::noop(),
          on_insert_parallel_intersection: Callback::noop(),
          on_insert_alternative_intersection: Callback::noop(),
          on_insert_loop_intersection: Callback::noop(),
          on_insert_element_pair_after_intersection: Callback::noop(),
        };

        if add_to_left.0 {
          self.branches.insert(branch_index.0, new_sequence);
        } else {
          self.branches.insert(branch_index.0 + 1, new_sequence);
        }
        true
      }
      AlternativeIntersectionMsg::PrependElementPair(branch_index) => {
        self.branches[branch_index.0]
          .elements
          .insert(0, Element::Transition(TransitionProps::default()));
        self.branches[branch_index.0]
          .elements
          .insert(1, Element::Step(StepProps::default()));
        true
      }
      // ---------------------------------
      // <<<--- Sequence operations --->>>
      // ---------------------------------
      AlternativeIntersectionMsg::SeqInsertElementPair(branch_index, transition_id) => {
        Sequence::insert_element_pair(&mut self.branches[branch_index.0], transition_id)
      }
      AlternativeIntersectionMsg::SeqInsertElementPairAfterIntersection(
        branch_index,
        intersection_id,
      ) => Sequence::insert_element_pair_after_intersection(
        &mut self.branches[branch_index.0],
        intersection_id,
      ),
      AlternativeIntersectionMsg::SeqInsertParallelIntersection(branch_index, step_id) => {
        Sequence::insert_parallel_intersection(&mut self.branches[branch_index.0], step_id)
      }
      AlternativeIntersectionMsg::SeqInsertAlternativeIntersection(branch_index, transition_id) => {
        Sequence::insert_alternative_intersection(&mut self.branches[branch_index.0], transition_id)
      }
      AlternativeIntersectionMsg::SeqInsertLoopIntersection(branch_index, step_id) => {
        Sequence::insert_loop_intersection(&mut self.branches[branch_index.0], step_id)
      }
    }
  }
}
