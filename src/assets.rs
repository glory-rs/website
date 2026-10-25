//! Media URLs are compiled identically into the browser and SSR generator.
//! The build computes this directory before compiling either target.
pub fn media(name: &str) -> String {
    format!("{}/{name}", option_env!("GLORY_WEBSITE_MEDIA").unwrap_or("/assets"))
}
