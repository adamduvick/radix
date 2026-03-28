use leptos::prelude::*;
use pith_ui::progress::*;

use crate::components::{extract_demo, DemoTabs};

const ROOT: &str = "relative h-3 w-full overflow-hidden rounded-full bg-slate-200";
const INDICATOR: &str = "h-full bg-accent-600 transition-all duration-500 ease-out";
const BUTTON: &str = "rounded-md border border-slate-300 px-3 py-1.5 text-sm \
    font-medium text-slate-700 hover:bg-slate-50";

// #region demo
#[component]
pub fn ProgressBasic() -> impl IntoView {
    let (value, set_value) = signal(33.0);

    view! {
        <div class="space-y-6 max-w-md">
            <div>
                <div class="mb-2 flex justify-between text-sm">
                    <span class="font-medium text-slate-700">"Progress"</span>
                    <span class="text-slate-500">{move || format!("{}%", value.get() as u32)}</span>
                </div>
                <Progress value=value.get() attr:class=ROOT>
                    <ProgressIndicator
                        attr:class=INDICATOR
                        attr:style=move || format!("width: {}%", value.get())
                    />
                </Progress>
            </div>

            <div class="flex gap-2">
                <button class=BUTTON on:click=move |_| set_value.set(0.0)>"0%"</button>
                <button class=BUTTON on:click=move |_| set_value.set(33.0)>"33%"</button>
                <button class=BUTTON on:click=move |_| set_value.set(66.0)>"66%"</button>
                <button class=BUTTON on:click=move |_| set_value.set(100.0)>"100%"</button>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn ProgressBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A determinate progress bar with interactive buttons to change the value."
        </p>

        <DemoTabs source=extract_demo(include_str!("progress_basic.rs"))>
            <ProgressBasic />
        </DemoTabs>
    }
}
