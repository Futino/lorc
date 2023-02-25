use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::lorc::generic::*;

mod props;
use props::Props;

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let Props { min, max, step } = props;
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = {
        let input_value_handle = input_value_handle.clone();

        log!(input_value.clone());

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            <Label>
                {input_value.clone()}
            </Label>
                { "My cautious input:" }
                <input onchange={on_change}
                    id="cautious-input"
                    type="range"
                    value={input_value.clone()}
                    min={min.to_owned()}
                    max={max.to_owned()}
                    step={step.to_owned()}
                />
        </>
    }
}
