use leptos::prelude::*;
use pith_ui::toggle::Toggle;

use crate::components::{extract_demo, DemoTabs};

const TOGGLE: &str = "inline-flex items-center justify-center rounded-md px-3 py-2 \
    text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors \
    data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700";

// #region demo
#[component]
pub fn ToggleBasic() -> impl IntoView {
    let (bold, set_bold) = signal(false);

    view! {
        <div class="flex items-center gap-2">
            <Toggle
                pressed=bold
                on_pressed_change=move |v| set_bold.set(v)
                attr:class=TOGGLE
                attr:aria-label="Toggle bold"
            >
                <span class="font-bold">"B"</span>
            </Toggle>
            <Toggle
                attr:class=TOGGLE
                attr:aria-label="Toggle italic"
            >
                <span class="italic">"I"</span>
            </Toggle>
            <Toggle
                attr:class=TOGGLE
                attr:aria-label="Toggle underline"
            >
                <span class="underline">"U"</span>
            </Toggle>
        </div>
    }
}
// #endregion demo

#[component]
pub fn ToggleBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A set of toggle buttons for text formatting with controlled and uncontrolled variants."
        </p>

        <DemoTabs source=extract_demo(include_str!("toggle_basic.rs"))>
            <ToggleBasic />
        </DemoTabs>
    }
}
