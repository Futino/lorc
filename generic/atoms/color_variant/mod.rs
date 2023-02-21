#[derive(PartialEq)]
pub enum ColorVariant {
    Primary,
    Secondary,
    Tertiary,

    OnPrimary,
    OnSecondary,
    OnTertiary,

    OnBackground,
    OnSurface,
    OnSurfaceVariant,
}

impl ColorVariant {
    pub fn as_string(&self) -> &'static str {
        match self {
            ColorVariant::Primary => "text-primary-light dark:text-primary-dark",
            ColorVariant::Secondary => "text-secondary-light dark:text-secondary-dark",
            ColorVariant::Tertiary => "text-tertiary-light dark:text-tertiary-dark",
            ColorVariant::OnPrimary => "text-primary-on-light dark:text-primary-on-dark",
            ColorVariant::OnSecondary => "text-secondary-on-light dark:text-secondary-on-dark",
            ColorVariant::OnTertiary => "text-tertiary-on-light dark:text-tertiary-on-dark",
            ColorVariant::OnBackground => "text-background-on-light dark:text-background-on-dark",
            ColorVariant::OnSurface => "text-surface-on-light dark:text-surface-on-dark",
            ColorVariant::OnSurfaceVariant => {
                "text-surface-variant-on-light dark:text-surface-variant-on-dark"
            }
        }
    }
}
