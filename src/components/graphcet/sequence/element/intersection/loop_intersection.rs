use yew::prelude::*;

use crate::components::{
  graphcet::sequence::{
    element::{
      intersection::{IntersectionId, TransitionId},
      step::StepProps,
      transition::{Transition, TransitionProps},
      Element, StepId,
    },
    Sequence, SequenceProps,
  },
  net_button::{NetButtonDirection, NetButtonProps},
  net_user_control::NetUserControl,
};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct LoopIntersectionProps {
  pub id: u128,
  pub sequence: SequenceProps,
  pub continue_transition: TransitionProps,
  pub exit_transition: TransitionProps,
}
impl Default for LoopIntersectionProps {
  fn default() -> Self {
    Self {
      id: 0,
      sequence: SequenceProps {
        elements: vec![
          Element::Step(StepProps::default()),
          Element::Transition(TransitionProps::default()),
        ],
        on_insert_element_pair: Callback::noop(),
        on_insert_parallel_intersection: Callback::noop(),
        on_insert_alternative_intersection: Callback::noop(),
        on_insert_element_pair_after_intersection: Callback::noop(),
        on_insert_loop_intersection: Callback::noop(),
      },
      continue_transition: TransitionProps::default(),
      exit_transition: TransitionProps::default(),
    }
  }
}

pub enum LoopIntersectionMsg {
  // <<<--- LoopIntersection operations --->>>
  PrependElementPair,
  // <<<--- Sequence operations --->>>
  SeqInsertElementPair(TransitionId),
  SeqInsertElementPairAfterIntersection(IntersectionId),
  SeqInsertParallelIntersection(StepId),
  SeqInsertAlternativeIntersection(TransitionId),
  SeqInsertLoopIntersection(StepId),
}

pub(crate) struct LoopIntersection {
  sequence: SequenceProps,
}
impl LoopIntersection {
  pub fn is_insertable(sequence: &SequenceProps, triggering_element: u128) -> bool {
    if let Some(pos) = sequence
      .elements
      .iter()
      .position(|x| triggering_element == x.get_id())
    {
      match sequence.elements.get(pos) {
        Some(Element::Step(_)) => {}
        _ => return false,
      }
      match sequence.elements.get(pos + 1) {
        Some(Element::Transition(_)) => {}
        _ => return false,
      }
      return true;
    } else {
      return false;
    }
  }
}
impl Component for LoopIntersection {
  type Message = LoopIntersectionMsg;

  type Properties = LoopIntersectionProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      sequence: _ctx.props().sequence.clone(),
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {<>
      <div
        key={ctx.props().id}
        class="intersection__loop-branch-seperation-line"/>
      <div class="intersection__grid-container">
        <div class="intersection__grid-item">
          <div class="intersection__content-wrapper">
            <div class="
              path__short 
              path__short--margin-left 
              intersection__entry-menu">
              <NetUserControl
                buttons={vec![
                  NetButtonProps {
                    direction: Some(NetButtonDirection::South),
                    button_text: "S".to_string(),
                    on_click: ctx.link().callback(move |_| LoopIntersectionMsg::PrependElementPair),
                  },
                ]}/>
            </div>
            <Sequence
              elements={self.sequence.elements.clone()}
              on_insert_element_pair={
                ctx
                .link()
                .callback(move |transition_id| LoopIntersectionMsg::SeqInsertElementPair(transition_id))
              }
              on_insert_parallel_intersection={
                ctx
                .link()
                .callback(move |step_id| LoopIntersectionMsg::SeqInsertParallelIntersection(step_id))
              }
              on_insert_element_pair_after_intersection={
                ctx
                .link()
                .callback(move |intersection_id| LoopIntersectionMsg::SeqInsertElementPairAfterIntersection(intersection_id))
              }
              on_insert_alternative_intersection={
                ctx
                .link()
                .callback(move |transition_id| LoopIntersectionMsg::SeqInsertAlternativeIntersection(transition_id))
              }
              on_insert_loop_intersection={
                ctx
                .link()
                .callback(move |step_id| LoopIntersectionMsg::SeqInsertLoopIntersection(step_id))
              }/>
          </div>
          <div class="intersection__vertical-fill-line"/>
          <div class="path__short path__short--margin-left"/>
        </div>
        <div class="intersection__grid-item">
          <div class="path__short path__short--margin-left"/>
          <div class="path__dynamic"/>
          <Transition
            transitions={ctx.props().continue_transition.transitions.clone()}
            id={ctx.props().continue_transition.id.clone()}
            buttons={vec![]}/>
          <div class="path__triangle_arrow_up"/>
          <div class="path__short path__short--margin-left"/>
        </div>
      </div>
      <div
        key={ctx.props().id}
        class="intersection__loop-branch-seperation-line"/>
        <Transition
          transitions={ctx.props().exit_transition.clone()}
          id={ctx.props().exit_transition.id.clone()}
          buttons={vec![]}/>
    </>}
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      // -----------------------------------------
      // <<<--- LoopIntersection operations --->>>
      // -----------------------------------------
      LoopIntersectionMsg::PrependElementPair => {
        self
          .sequence
          .elements
          .insert(0, Element::Step(StepProps::default()));
        if self.sequence.elements.len() > 1 {
          self
            .sequence
            .elements
            .insert(1, Element::Transition(TransitionProps::default()));
        }
        true
      }
      // ---------------------------------
      // <<<--- Sequence operations --->>>
      // ---------------------------------
      LoopIntersectionMsg::SeqInsertElementPair(transition_id) => {
        Sequence::insert_element_pair(&mut self.sequence, transition_id)
      }
      LoopIntersectionMsg::SeqInsertElementPairAfterIntersection(intersection_id) => {
        Sequence::insert_element_pair_after_intersection(&mut self.sequence, intersection_id)
      }
      LoopIntersectionMsg::SeqInsertParallelIntersection(step_id) => {
        Sequence::insert_parallel_intersection(&mut self.sequence, step_id)
      }
      LoopIntersectionMsg::SeqInsertAlternativeIntersection(transition_id) => {
        Sequence::insert_alternative_intersection(&mut self.sequence, transition_id)
      }
      LoopIntersectionMsg::SeqInsertLoopIntersection(step_id) => {
        Sequence::insert_loop_intersection(&mut self.sequence, step_id)
      }
    }
  }
}
