use yew::prelude::*;

use crate::{
  components::{
    graphcet::sequence::{
      element::{
        intersection::{BranchIndex, TransitionId},
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

use super::{AddToLeft, IntersectionId, IntersectionProps, IntersectionType};

#[derive(Clone, PartialEq, Properties, Debug)]
pub(crate) struct ParallelIntersecionProps {
  pub exit_transition: TransitionProps,
  pub id: u128,
  pub branches: Vec<SequenceProps>,

  pub on_prepend_element_pair: Callback<BranchIndex>,
  pub on_insert_element_pair: Callback<(BranchIndex, TransitionId)>,
  pub on_append_element_pair: Callback<BranchIndex>,

  pub on_attach_element_pair_to_intersection: Callback<()>,
  pub on_pass_attach_element_pair_to_intersection: Callback<(BranchIndex, IntersectionId)>,

  pub on_insert_parallel_intersection: Callback<(BranchIndex, StepId)>,

  pub on_add_branch: Callback<(BranchIndex, AddToLeft)>,
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
        on_insert_element_pair: Callback::noop(),
        on_insert_parallel_intersection: Callback::noop(),
        on_attach_element_pair_to_intersection: Callback::noop(),
      }],
      on_prepend_element_pair: Callback::noop(),
      on_append_element_pair: Callback::noop(),
      on_insert_element_pair: Callback::noop(),

      on_attach_element_pair_to_intersection: Callback::noop(),

      on_insert_parallel_intersection: Callback::noop(),

      on_add_branch: Callback::noop(),
      on_pass_attach_element_pair_to_intersection: Callback::noop(),
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
        on_insert_element_pair: Callback::noop(),
        on_insert_parallel_intersection: Callback::noop(),
        on_attach_element_pair_to_intersection: Callback::noop(),
      };

      let new_parallel_intersection = IntersectionProps {
        id: UuidService::new_index(),
        intersection_type: IntersectionType::ParallelBranches(triggering_transition),
        branches: vec![
          new_branch,
          SequenceProps {
            elements: vec![Element::Step(StepProps::default())],
            on_insert_element_pair: Callback::noop(),
            on_insert_parallel_intersection: Callback::noop(),
            on_attach_element_pair_to_intersection: Callback::noop(),
          },
        ],
        on_attach_element_pair_to_intersection: Callback::noop(),
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
                <div class="
                  path__short 
                  path__short--margin-left 
                  intersection__entry-menu">
                  <NetUserControl
                    buttons={vec![
                      NetButtonProps {
                        direction: Some(NetButtonDirection::West),
                        button_text: "B".to_string(),
                        on_click: ctx.props().on_add_branch.reform(move |_| (BranchIndex(index), AddToLeft(true))),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::South),
                        button_text: "S".to_string(),
                        on_click: ctx.props().on_prepend_element_pair.reform(move |_| BranchIndex(index)),
                      },
                      NetButtonProps {
                        direction: Some(NetButtonDirection::East),
                        button_text: "B".to_string(),
                        on_click: ctx.props().on_add_branch.reform(move |_| (BranchIndex(index), AddToLeft(false))),
                      },
                    ]}/>
                </div>
                <Sequence
                  elements={item.elements.clone()}
                  on_insert_element_pair={
                    ctx
                    .props().on_insert_element_pair
                    .reform(move |transition_id| (BranchIndex(index), transition_id))}
                  on_insert_parallel_intersection={
                    ctx
                    .props()
                    .on_insert_parallel_intersection
                    .reform(move |step_id| (BranchIndex(index), step_id))}
                  on_attach_element_pair_to_intersection={
                    ctx
                    .props()
                    .on_pass_attach_element_pair_to_intersection
                    .reform(move |intersection_id| (BranchIndex(index), intersection_id))
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
                      on_click: ctx.props().on_append_element_pair.reform(move |_| BranchIndex(index)),
                    },
                  ]}/>
            </div>
              </div>
            </div>
          }
        })}
      </div>
      <div class="intersection__parallel-branch-seperation-line"/>
        <Transition
          transitions={ctx.props().exit_transition.transitions.clone()}
          id={ctx.props().exit_transition.id.clone()}
          buttons={vec![
            NetButtonProps {
              direction: Some(NetButtonDirection::South),
              button_text: "S".to_string(),
              on_click:
                ctx
                .props()
                .on_attach_element_pair_to_intersection
                .reform(|_| ())
            },
          ]}/>
    </>}
  }
}
