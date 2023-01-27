use yew::prelude::*;

// pub enum Variants {
//     Action,
//     Link,
// }

#[derive(PartialEq)]
pub struct ButtonOptions {
    pub onclick: Callback<MouseEvent>,
    pub href: Option<String>,
}
