use yew::prelude::*;

use crate::{
  components::{
    graphcet::sequence::{
      element::{
        intersection::{AddToLeft, BranchIndex, IntersectionId, TransitionId},
        step::StepProps,
        transition::{Transition, TransitionProps},
        Element, StepId,
      },
      Sequence, SequenceProps,
    },
    net_button::{NetButtonDirection, NetButtonProps},
    net_user_control::NetUserControl,
  },
  services::uuid_service::UuidService,
};

#[derive(Clone, PartialEq, Properties, Debug)]
pub(crate) struct ParallelIntersecionProps {
  /// If a alternative intersection follows the parallel intersection, an exit transition is forbidden.
  pub exit_transition: Option<TransitionProps>,
  pub id: u128,
  pub branches: Vec<SequenceProps>,
  pub on_insert_element_pair_after_intersection: Callback<IntersectionId>,
}

impl Default for ParallelIntersecionProps {
  fn default() -> Self {
    Self {
      exit_transition: Option::Some(TransitionProps::default()),
      id: UuidService::new_index(),
      branches: vec![SequenceProps {
        elements: vec![
          Element::Step(Default::default()),
          Element::Transition(Default::default()),
        ],
        on_insert_element_pair: Callback::noop(),
        on_insert_parallel_intersection: Callback::noop(),
        on_insert_alternative_intersection: Callback::noop(),
        on_insert_element_pair_after_intersection: Callback::noop(),
        on_insert_loop_intersection: Callback::noop(),
      }],
      on_insert_element_pair_after_intersection: Callback::noop(),
    }
  }
}

pub(crate) struct ParallelIntersection {
  branches: Vec<SequenceProps>,
}

pub enum ParallelIntersectionMsg {
  // <<<--- ParallelIntersection operations --->>>
  PrependElementPair(BranchIndex),
  AppendElementPair(BranchIndex),
  AddBranch(BranchIndex, AddToLeft),
  // <<<--- Sequence operations --->>>
  SeqInsertElementPair(BranchIndex, TransitionId),
  SeqInsertElementPairAfterIntersection(BranchIndex, IntersectionId),
  SeqInsertParallelIntersection(BranchIndex, StepId),
  SeqInsertAlternativeIntersection(BranchIndex, TransitionId),
  SeqInsertLoopIntersection(BranchIndex, StepId),
}

impl Component for ParallelIntersection {
  type Message = ParallelIntersectionMsg;
  type Properties = ParallelIntersecionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      branches: _ctx.props().branches.clone(),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let intersection_id = IntersectionId(ctx.props().id);
    html! {<>
      <div class="intersection__parallel-branch-seperation-line"/>
      <div class="intersection__grid-container" key={ctx.props().id.to_string()+"_grid-container"}>
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
                        on_click: ctx.link().callback(move |_| ParallelIntersectionMsg::AddBranch(BranchIndex(index), AddToLeft(true))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::South),
                        button_text: "S".to_string(),
                        on_click: ctx.link().callback(move |_| ParallelIntersectionMsg::PrependElementPair(BranchIndex(index))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::East),
                        button_text: "B".to_string(),
                        on_click: ctx.link().callback(move |_| ParallelIntersectionMsg::AddBranch(BranchIndex(index), AddToLeft(false))),
                      },
                    ]}/>
                </div>
                <Sequence
                  elements={item.elements.clone()}
                  on_insert_element_pair={
                    ctx
                    .link()
                    .callback(move |transition_id| ParallelIntersectionMsg::SeqInsertElementPair(BranchIndex(index), transition_id))}
                  on_insert_element_pair_after_intersection={
                    ctx
                    .link()
                    .callback(move |intersection_id| ParallelIntersectionMsg::SeqInsertElementPairAfterIntersection(BranchIndex(index), intersection_id))
                  }
                  on_insert_parallel_intersection={
                    ctx
                    .link()
                    .callback(move |step_id| ParallelIntersectionMsg::SeqInsertParallelIntersection(BranchIndex(index), step_id))}
                  on_insert_alternative_intersection={
                    ctx
                    .link()
                    .callback(move |transition_id| ParallelIntersectionMsg::SeqInsertAlternativeIntersection(BranchIndex(index), transition_id))
                  }
                  on_insert_loop_intersection={
                    ctx
                    .link()
                    .callback(move |step_id| ParallelIntersectionMsg::SeqInsertLoopIntersection(BranchIndex(index), step_id))
                  }/>
              </div>
              <div class="intersection__vertical-fill-line"/>
              <div class="intersection__hover-container">
              <div class="
                path__short 
                path__short--margin-left 
                intersection__entry-menu">
                <NetUserControl
                  buttons={vec![
                    NetButtonProps {
                      direction: Some(NetButtonDirection::North),
                      button_text: "S".to_string(),
                      on_click: ctx
                      .link()
                      .callback(move |_| ParallelIntersectionMsg::AppendElementPair(BranchIndex(index)))
                    },
                  ]}/>
            </div>
              </div>
            </div>
          }
        })}
      </div>
      <div class="intersection__parallel-branch-seperation-line"/>{
        match &ctx.props().exit_transition {
          Some(exit_transition) => html! {
            <Transition
              transitions={exit_transition.clone()}
              id={exit_transition.id.clone()}
              buttons={vec![
                NetButtonProps {
                  direction: Some(NetButtonDirection::South),
                  button_text: "S".to_string(),
                  on_click:
                    ctx
                    .props()
                    .on_insert_element_pair_after_intersection
                    .reform(move |_| intersection_id)
                },
              ]}/>
          },
          None => html! {
            <div class="
              path__short 
              path__short--margin-left 
              intersection__entry-menu">
              <NetUserControl
                buttons={vec![
                  NetButtonProps {
                    direction: Some(NetButtonDirection::South),
                    button_text: "S".to_string(),
                    on_click:
                      ctx
                      .props()
                      .on_insert_element_pair_after_intersection
                      .reform(move |_| intersection_id)
                  },
                ]}/>
            </div>
          }
        }
      }
    </>}
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      // ---------------------------------------------
      // <<<--- ParallelIntersection operations --->>>
      // ---------------------------------------------
      ParallelIntersectionMsg::AddBranch(branch_index, add_to_left) => {
        let new_sequence = SequenceProps {
          elements: vec![Element::Step(StepProps::default())],
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
      ParallelIntersectionMsg::PrependElementPair(branch_index) => {
        self.branches[branch_index.0]
          .elements
          .insert(0, Element::Step(StepProps::default()));
        if self.branches[branch_index.0].elements.len() > 1 {
          self.branches[branch_index.0]
            .elements
            .insert(1, Element::Transition(TransitionProps::default()));
        }
        true
      }
      ParallelIntersectionMsg::AppendElementPair(branch_index) => {
        Sequence::append_element_pair_to_intersection(&mut self.branches[branch_index.0])
      }
      // ---------------------------------
      // <<<--- Sequence operations --->>>
      // ---------------------------------
      ParallelIntersectionMsg::SeqInsertElementPair(branch_index, transition_id) => {
        Sequence::insert_element_pair(&mut self.branches[branch_index.0], transition_id)
      }
      ParallelIntersectionMsg::SeqInsertElementPairAfterIntersection(
        branch_index,
        intersection_id,
      ) => Sequence::insert_element_pair_after_intersection(
        &mut self.branches[branch_index.0],
        intersection_id,
      ),
      ParallelIntersectionMsg::SeqInsertParallelIntersection(branch_index, step_id) => {
        Sequence::insert_parallel_intersection(&mut self.branches[branch_index.0], step_id)
      }
      ParallelIntersectionMsg::SeqInsertAlternativeIntersection(branch_index, transition_id) => {
        Sequence::insert_alternative_intersection(&mut self.branches[branch_index.0], transition_id)
      }
      ParallelIntersectionMsg::SeqInsertLoopIntersection(branch_index, step_id) => {
        Sequence::insert_loop_intersection(&mut self.branches[branch_index.0], step_id)
      }
    }
  }
}
