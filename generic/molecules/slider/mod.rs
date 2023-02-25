use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{prelude::*, virtual_dom::Listener};

use crate::lorc::generic::*;

mod props;
use props::Props;

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let Props { min, max, step } = props;
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_input = {
        log!(input_value.clone());
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: InputEvent| {
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

    // Parse min prop to u32
    let max_number = match min.parse::<u32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing input1: {:?}", e);
            69
        }
    };
    // Parse max prop to u32
    let min_number = match max.parse::<u32>() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing input2: {:?}", e);
            69
        }
    };

    // Calculate middle
    let middle: String = ((min_number + max_number) / 2).to_string();

    html! {
        <>
            <h1>
                {
                    if input_value == "" {
                    &middle
                }
                else {
                    &input_value
                }
                }
            </h1>
                { "My cautious input:" }
                <input oninput={on_input}
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
