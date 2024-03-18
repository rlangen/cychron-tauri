use web_sys::HtmlTextAreaElement;
use yew::{html::IntoPropValue, prelude::*};

pub struct Transition {
    transitions: String,
}

#[derive(Clone, PartialEq, Properties, Default, Debug)]
pub struct TransitionProps {
    pub transitions: String,
}
impl IntoPropValue<String> for TransitionProps {
    fn into_prop_value(self) -> String {
        // Convert self into a Vec<sequence::Element> here
        self.transitions
    }
}

pub enum Msg {
    TransitionsUpdateEvent(String),
}

impl Component for Transition {
    type Message = Msg;
    type Properties = TransitionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Transition {
            transitions: _ctx.props().transitions.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TransitionsUpdateEvent(new_text) => {
                self.transitions = new_text;
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
                    margin-left: 10px;
            ">
                <div
                    style="
                        display: flex; 
                        flex-direction: column; 
                        align-items: center;
                        margin-right: 5px
                ">
                    <div
                        style="
                            width: 2px; 
                            height: 15px; 
                            background-color: black;" 
                    />
                    <div
                        style="
                            width: 30px; 
                            height: 4px; 
                            background-color: black;" 
                    />
                    <div
                        style="
                            width: 2px; 
                            height: 30px; 
                            background-color: black;" 
                    />
                </div>
                <div
                    style="
                        width: 200px;
                        height: 30px; 
                        background-color: transparent;
                        border-style: none;
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
                        value={self.transitions.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::TransitionsUpdateEvent(input.value())
                        })}
                    />
                </div>
            </div>
        }
    }
}
