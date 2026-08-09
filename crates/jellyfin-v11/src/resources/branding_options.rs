use core_macros::resource;

/// Branding — login disclaimer, custom CSS, splashscreen.
#[resource(
    sync = singleton,
    case = pascal,
    read = get("/Branding/Configuration"),
    update = post("/System/Configuration/Branding"),
)]
pub struct BrandingOptions {
    /// Login Disclaimer
    pub login_disclaimer: Option<String>,
    /// Custom Css
    pub custom_css: Option<String>,
    /// Splashscreen Enabled
    pub splashscreen_enabled: bool,
}
