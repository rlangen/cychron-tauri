use yew::prelude::*;

use super::transition::{Transition, TransitionProps};
use crate::graphcet::sequence::{Sequence, SequenceProps};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct IntersectionProps {
    pub branches: Vec<SequenceProps>,
    pub intersection_type: IntersectionType,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum IntersectionType {
    ParallelBranches(TransitionProps),
    #[default]
    AlternativeBranches,
    LoopBranches,
}

pub struct Intersection {}

impl Component for Intersection {
    type Message = ();
    type Properties = IntersectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let line_width = if ctx.props().branches.len() > 1 {
            (ctx.props().branches.len() - 1) * 354 + 50
        } else {
            50
        };

        html! {
            <>
                {
                    match ctx.props().intersection_type {
                        IntersectionType::ParallelBranches(_) => html! {
                            <>
                                <div
                                    style={format!("
                                        height: 2px;
                                        width: {}px;
                                        background-color: black;", line_width)}/>
                                <div style="height: 2px;"/>
                                <div
                                    style={format!("
                                        height: 2px;
                                        width: {}px;
                                        background-color: black;", line_width)}/>
                            </>
                        },
                        IntersectionType::AlternativeBranches => html! {
                            <div style="height: 70px;"/>
                        },
                        IntersectionType::LoopBranches => html! {
                            <div style="height: 70px;"/>
                        },
                    }
                }
                <div class="intersection__grid-container">
                    { for ctx.props().branches.iter().enumerate().map(|(index, item)| {
                        html! {
                            <div class="intersection__grid-item">
                                <div class="intersection__content-wrapper">
                                    <div class="path__short"/>
                                    <Sequence
                                        key={index.clone()}
                                        elements={item.elements.clone()} />
                                </div>
                                <div class="intersection__vertical-fill-line"/>
                                <div class="path__short"/>
                            </div>
                        }
                    })}
                </div>
                {
                    match &ctx.props().intersection_type {
                        IntersectionType::ParallelBranches(transition_props) => html! {
                            <>
                                <div
                                    style={format!("
                                        height: 2px;
                                        width: {}px;
                                        background-color: black;", line_width)}/>
                                <div style="height: 2px;"/>
                                <div
                                    style={format!("
                                        height: 2px;
                                        width: {}px;
                                        background-color: black;", line_width)}/>
                                <Transition transitions={transition_props.clone()}/>
                            </>
                        },
                        IntersectionType::AlternativeBranches => html! {
                            <div style="height: 70px;"/>
                        },
                        IntersectionType::LoopBranches => html! {
                            <div style="height: 70px;"/>
                        },
                    }
                }
            </>
        }
    }
}
