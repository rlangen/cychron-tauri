use yew::{html, Component, Context, Html};

pub struct HoverControl;

impl Component for HoverControl {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="hover_control__container">
                <div class= "hover_control__add-direction hover_control__add-direction--right">
                    {"→"}
                </div>
                <div class= "hover_control__add-direction hover_control__add-direction--down">
                    {"↓"}
                </div>
                <button class="hover_control__button hover_control__button--add-parallel">
                    { "P" }
                </button>
                <button class="hover_control__button hover_control__button--add-alternative">
                    { "A" }
                </button>
                <button class="hover_control__button hover_control__button--add-step">
                    { "S" }
                </button>
                <button class="hover_control__button hover_control__button--add-delete">
                    { "🗑" }
                </button>
            </div>
        }
    }
}
