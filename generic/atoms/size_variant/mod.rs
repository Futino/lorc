#[derive(PartialEq)]
pub enum SizeVariant {
    Large,
    Medium,
    Small,
}

/*
impl SizeVariant {
    pub fn as_string(&self) -> &'static str {
        match self {
            SizeVariant::Large => "text-8xl",
            SizeVariant::Medium => "text-7xl",
            SizeVariant::Small => "text-6xl",
        }
    }
}
*/
