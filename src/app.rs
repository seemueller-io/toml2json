use leptos::*;

use crate::components::conversion_interface::ConversionInterface;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>"TOML to JSON Converter"</h1>
        <ConversionInterface />
    }
}
