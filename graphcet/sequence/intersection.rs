use yew::prelude::*;

use crate::graphcet::sequence::{Sequence, SequenceProps};

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct IntersectionProps {
    pub branches: Vec<SequenceProps>,
    pub intersection_type: IntersectionType,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum IntersectionType {
    #[default]
    ParallelBranches,
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
                        IntersectionType::ParallelBranches => html! {
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
                <div style="
                    display: inline-flex;
                    flex_direction: column;
                    align_items: left;
                    margin-left: 0px;
                ">
                    { for ctx.props().branches.iter().enumerate().map(|(index, item)| {
                        html! {
                            <div
                                style="
                                    display: flex;
                                    flex_direction: row;
                                    align_items: left;">
                                if index != 0 {
                                    <div style="width: 70px;"/>
                                }
                                <div>
                                    <div
                                        style="
                                            width: 2px; 
                                            height: 15px;
                                            margin-left: 24px; 
                                            background-color: black;" />
                                    <Sequence
                                        key={index.clone()}
                                        elements={item.elements.clone()} />
                                    <div
                                        style="
                                            width: 2px; 
                                            height: 100%;
                                            margin-left: 24px; 
                                            background-color: black;" />
                                </div>
                            </div>
                        }
                    })}
                </div>
                {
                    match ctx.props().intersection_type {
                        IntersectionType::ParallelBranches => html! {
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
            </>
        }
    }
}
