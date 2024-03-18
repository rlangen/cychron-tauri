use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub struct Step {
    action_name: String,
}

pub enum Msg {
    ActionNameUpdateEvent(String),
}

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct StepProps {
    pub index: usize,
    pub action_name: String,
}

impl Component for Step {
    type Message = Msg;
    type Properties = StepProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            action_name: _ctx.props().action_name.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ActionNameUpdateEvent(new_text) => {
                self.action_name = new_text;
                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                style="
                    display: flex; 
                    flex-direction: row; 
                    align-items: center;
                    margin-left: 0;
            ">
                <div
                    style="
                        width: 44px; 
                        height: 44px; 
                        background-color: white;
                        border-color: black;
                        border-style: solid;
                        border-width: 3px;
                        margin: 0 0;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                ">
                    {ctx.props().index}
                </div>
                <div
                    style="
                        width: 30px; 
                        height: 3px; 
                        background-color: black;" 
                />
                <div
                    style="
                        width: 200px;
                        height: 30px; 
                        background-color: white;
                        border-color: black;
                        border-style: solid;
                        border-width: 3px;
                        margin: 0 0;
                        display: flex;
                        align-items: center;
                ">
                    <textarea
                        style="
                            width: 100%;
                            height: 90%;
                            border: none;
                            background-color: transparent;
                            font-size: 15px;
                            font-weight: normal;
                            text-align: left;
                            padding-left: 5px;
                            padding-top: 5px;
                            resize: none;
                            auto-correct: off;"
                        spellcheck="false"
                        value={self.action_name.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::ActionNameUpdateEvent(input.value())
                        })}
                    />
                </div>
            </div>
        }
    }
}
