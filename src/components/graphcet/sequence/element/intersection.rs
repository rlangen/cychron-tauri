use yew::prelude::*;

use self::parallel_intersection::OnAddStepAndTransitionData;

use super::transition::TransitionProps;
use crate::{
    components::graphcet::sequence::{
        element::{step::StepProps, Element},
        SequenceProps,
    },
    services::{logging_service::Log, uuid_service::UuidService},
};

mod parallel_intersection;
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
    /// (Branch index, Transition id)
    AddStepAndTransition(OnAddStepAndTransitionData),
    /// (Transition id)
    AppendTransitionAndStep(usize),
}

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
        let line_width = if ctx.props().branches.len() > 1 {
            (ctx.props().branches.len() - 1) * 354 + 50
        } else {
            50
        };

        html! {
            <>
                {match &ctx.props().intersection_type {
                    IntersectionType::ParallelBranches(exit_transition) => html! {
                        <ParallelIntersection
                                branches={self.branches.clone()}
                                exit_transition={exit_transition.clone()}
                                line_width={line_width.clone()}
                                id={UuidService::new_index()}
                                on_add_step_and_transition={ctx.link().callback(|on_add_step_and_transition_data|IntersectionMsg::AddStepAndTransition(on_add_step_and_transition_data))}
                                on_append_transition_and_step={ctx.link().callback(|branch_index|IntersectionMsg::AppendTransitionAndStep(branch_index))}/>
                    },
                    IntersectionType::AlternativeBranches => html! {
                        <AlternativeIntersection
                            branches={self.branches.clone()}
                            line_width={line_width.clone()}
                            id={UuidService::new_index()}
                            on_append_step_and_transition={
                                ctx
                                .link()
                                .callback(|on_add_step_and_transition_data|IntersectionMsg::AddStepAndTransition(on_add_step_and_transition_data))}/>
                    },
                    IntersectionType::LoopBranches(continue_transition, exit_transition) => html! {
                        <LoopIntersection
                            branches={self.branches.clone()}
                            continue_transition={continue_transition.clone()}
                            exit_transition={exit_transition.clone()}
                            line_width={line_width.clone()}
                            id={UuidService::new_index()}
                            on_append_step_and_transition={
                                ctx
                                .link()
                                .callback(|on_add_step_and_transition_data|IntersectionMsg::AddStepAndTransition(on_add_step_and_transition_data))}/>
                    },
                }}
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IntersectionMsg::AddStepAndTransition(on_add_step_and_transition_data) => {
                let branch_index = on_add_step_and_transition_data.branch_index;
                let transition_id = on_add_step_and_transition_data.transition_id;

                if let Some(pos) = ctx.props().branches[branch_index]
                    .elements
                    .iter()
                    .position(|x| transition_id == x.get_id())
                {
                    let id = UuidService::new_index();
                    let new_element = Element::Step(StepProps {
                        id,
                        action_name: "".to_string(),
                    });
                    self.branches[branch_index]
                        .elements
                        .insert(pos + 1, new_element);

                    let id = UuidService::new_index();
                    let new_transition = Element::Transition(TransitionProps {
                        id,
                        transitions: "".to_string(),
                        on_add_step: ctx.link().callback(move |_| {
                            IntersectionMsg::AddStepAndTransition(OnAddStepAndTransitionData {
                                branch_index,
                                transition_id: id,
                            })
                        }),
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
                #[cfg(debug_assertions)]
                Log::debug::<Self>("");

                let id = UuidService::new_index();
                let new_transition = Element::Transition(TransitionProps {
                    id,
                    transitions: "".to_string(),
                    on_add_step: ctx.link().callback(move |_| {
                        IntersectionMsg::AddStepAndTransition(OnAddStepAndTransitionData {
                            branch_index,
                            transition_id: id,
                        })
                    }),
                });
                self.branches[branch_index].elements.push(new_transition);

                let id = UuidService::new_index();
                let new_step = Element::Step(StepProps {
                    id,
                    action_name: "".to_string(),
                });
                self.branches[branch_index].elements.push(new_step);

                return true;
            }
        }
    }
}
