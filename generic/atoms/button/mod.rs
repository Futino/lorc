use yew::prelude::*;

mod props;
use props::Props;

pub mod button_options;
use button_options::ButtonOptions;
pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = &ctx.props().options.onclick;
        html! {
            <button {onclick} class={format!("overflow-hidden relative justify-center items-center group {}",ctx.props().class)}>
                {for ctx.props().children.iter()}
            </button>
        }
    }
}
