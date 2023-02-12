use yew::Properties;

use super::IconType;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub class: String,
    pub icon: IconType,
    pub height: String,
    pub width: String,
    #[prop_or_default]
    pub color: String,
}
