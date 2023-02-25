use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub min: String,
    pub max: String,
    #[prop_or("1".to_string())]
    pub step: String,
}
