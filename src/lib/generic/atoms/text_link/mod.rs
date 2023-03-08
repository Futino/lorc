use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::generic::atoms::*;

#[function_component]
pub fn TextLink(props: &Props) -> Html {
    let Props { children, href } = props;
    html! {
                        <a class="inline text-secondary-light dark:text-secondary-dark hover:text-tertiary-light dark:hover:text-tertiary-dark"
                            href={href.to_owned()}>
                            {for children.iter()}
                        </a>
    }
}
