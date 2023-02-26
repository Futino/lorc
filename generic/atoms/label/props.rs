use yew::{Children, Properties};

use crate::lorc::atoms::{ColorVariant, SizeVariant};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub id: String,
    pub children: Children,
    #[prop_or(SizeVariant::Medium)]
    pub size: SizeVariant,
    // This is for setting the alignment, font-weight, and incase the user wants to override the padding of the header. Padding is the only thing that I know of that can be overridden.
    #[prop_or(String::from("text-center font-bold"))]
    pub class: String,
    #[prop_or(ColorVariant::OnBackground)]
    pub color: ColorVariant,
}
