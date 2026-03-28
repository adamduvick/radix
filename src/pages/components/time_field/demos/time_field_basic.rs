use leptos::prelude::*;
use pith_ui::time_field::*;

use crate::components::{extract_demo, DemoTabs};

const INPUT: &str = "inline-flex items-center rounded-lg border border-slate-300 px-3 py-2 \
    text-sm font-mono text-slate-700 \
    focus-within:border-accent-500 focus-within:ring-1 focus-within:ring-accent-500 \
    [&_[data-pith-time-segment]]:outline-none \
    [&_[data-pith-time-segment]:focus]:bg-accent-100 \
    [&_[data-pith-time-segment]:focus]:text-accent-700 \
    [&_[data-pith-time-segment]]:rounded \
    [&_[data-pith-time-segment]]:px-0.5";

// #region demo
#[component]
pub fn TimeFieldBasic() -> impl IntoView {
    let (time, set_time) = signal::<Option<NaiveTime>>(None);

    view! {
        <div class="space-y-4 max-w-xs">
            <div>
                <label class="mb-1.5 block text-sm font-medium text-slate-700">"Meeting time"</label>
                <TimeField
                    value=MaybeProp::derive(move || time.get())
                    on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
                >
                    <TimeFieldInput
                        attr:class=INPUT
                        aria_label="Meeting time"
                    />
                </TimeField>
                <p class="mt-2 text-xs text-slate-500">
                    {move || match time.get() {
                        Some(t) => format!("Selected: {}", t.format("%I:%M %p")),
                        None => "No time selected".to_string(),
                    }}
                </p>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn TimeFieldBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A controlled time field with segmented hour, minute, and period inputs."
        </p>

        <DemoTabs source=extract_demo(include_str!("time_field_basic.rs"))>
            <TimeFieldBasic />
        </DemoTabs>
    }
}
