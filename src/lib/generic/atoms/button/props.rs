use std::borrow::Borrow;

use super::ButtonOptions;
use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
    pub options: ButtonOptions,
}
