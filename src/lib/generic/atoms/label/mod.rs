use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::lorc::atoms::{ColorVariant, SizeVariant};

// The purpose of these headers is to increase uniformity, decrease recalling arbitrary numbers for sizes (H1,H2,H3 sizes),
// and writing cleaner code (not using "text-background-on-light dark:text-background-on-dark", but instead using enums).

/* Use docs
<H1 color=ColorVariant::OnBackground class="p-20 text-left font-superbold">
    {"Text here...."}
</H1>

<H2 color=ColorVariant::OnSurface class="text-right font-normal">
    {"Text here...."}
</H2>

<H3 color=ColorVariant::OnSurfaceVariant class="font-bold">
    {"Text here...."}
</H3>
*/

#[function_component()]
pub fn Label(props: &Props) -> Html {
    let css_size = match props.size {
        SizeVariant::Large => "label-large".to_string(),
        SizeVariant::Medium => "label-medium".to_string(),
        SizeVariant::Small => "label-small".to_string(),
    };

    let css_color = props.color.as_string();

    html! {
        <div id={props.id.to_owned()} class={format!("p-3 {} {} {}",props.class, css_color, css_size)}>
            {for props.children.iter()}
        </div>
    }
}
