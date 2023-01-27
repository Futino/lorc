use yew::prelude::*;

mod props;
use props::Props;

mod button_options;
use button_options::ButtonOptions;
pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = &ctx.props().option.onclick;
        html! {
            <button {onclick} class="inline-flex overflow-hidden relative justify-center items-center py-3 px-6 font-bold rounded-md shadow-2xl group text-primary-on-light dark:text-primary-on-dark hover:text-secondary-light dark:hover:text-secondary-dark">
                <span class="absolute inset-0 w-full h-full bg-gradient-to-br from-primary-light to-secondary-light dark:from-primary-dark dark:to-secondary-dark opacity-100 transition duration-300 ease-out group-hover:opacity-0 "></span>
                <span class="absolute inset-0 w-full h-full bg-gradient-to-br from-secondary-light to-primary-light opacity-0 transition duration-300 ease-out group-hover:opacity-100 "></span>
                <span class="relative ">{for ctx.props().children.iter()}</span>
            </button>
        }
    }
}
