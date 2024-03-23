use yew::prelude::*;

mod step;
mod transition;
use step::Step;
use transition::Transition;

pub struct Row;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct RowProps {
    pub uid: usize,
    pub on_clicked: Callback<usize>,
    pub index: usize,
    pub action_name: String,
    pub transitions: String,
}

impl Component for Row {
    type Message = ();
    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let on_clicked = ctx.props().on_clicked.clone();
        let returned_index = ctx.props().index.clone();
        let on_clicked = ctx
            .props()
            .on_clicked
            .reform(move |_| returned_index.clone());

        html! {
            <div
                style="
                    display: flex; 
                    flex-direction: column; 
                    align-items: flex-start;
            ">
                <Step key={ctx.props().uid.clone()} index={ctx.props().index.clone()} action_name={ctx.props().action_name.clone()}/>
                <Transition transitions={ctx.props().transitions.clone()}/>
                <button
                    style="
                        width: 20px; 
                        height: 20px;
                        border-radius: 50%;
                        border: none;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        background-color: blue;                        
                        padding: 0px;
                        margin: 0px;
                        margin-left: 15px;"
                        onclick={on_clicked}
                >
                    <span
                        style="
                            color: white;
                            font-size: 20px;
                            font-weight: bold;
                            padding: 0px;
                            margin: 0px;
                    ">
                        {"+"}
                    </span>
                </button>
            </div>
        }
    }
}
