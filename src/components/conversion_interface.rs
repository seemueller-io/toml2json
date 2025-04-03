use leptos::*;
use crate::converters::toml2json::convert_toml_to_json;

#[component]
pub fn ConversionInterface() -> impl IntoView {
    let (toml_text, set_toml_text) = create_signal("".to_string());
    let (json_text, set_json_text) = create_signal("".to_string());

    let convert_toml_to_json = move |_| {
        let toml_value = toml_text.get();

        let converted = convert_toml_to_json(toml_value.as_str());
        if toml_value.is_empty() {
            set_json_text.set("// Paste TOML input above and click 'Convert to JSON'".to_string());
        } else {
            set_json_text.set(converted.expect("failed to convert TOML to JSON"));
        }
    };

    view! {
        <div class="container">

            <div class="input-section">
                <h2>"TOML Input"</h2>
                <textarea
                    placeholder="Paste your TOML here"
                    rows="10"
                    cols="80"
                    class="toml-input"
                    on:input=move |ev| { set_toml_text.set(event_target_value(&ev)); }
                    prop:value=toml_text
                />
            </div>

            <div class="button-section">
                <button on:click=convert_toml_to_json>"Convert to JSON"</button>
            </div>

            <div class="output-section">
                <h2>"JSON Output"</h2>
                <textarea
                    placeholder="JSON output will appear here"
                    rows="10"
                    cols="80"
                    class="json-output"
                    readonly=true
                    prop:value=json_text
                />
            </div>
        </div>
    }
}