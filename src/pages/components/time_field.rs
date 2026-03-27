use leptos::prelude::*;
use pith_ui::time_field::*;

use crate::components::*;

#[component]
pub fn TimeFieldPage() -> impl IntoView {
    let (time, set_time) = signal::<Option<NaiveTime>>(None);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Time Field"
                description="A segmented time input with individual fields for hours, minutes, seconds, and period."
                features=&["Accessible", "Segmented spinbutton input", "12h & 24h formats", "Configurable granularity", "Keyboard control"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-4 max-w-xs">
                        <div>
                            <label class="mb-1.5 block text-sm font-medium text-slate-700">"Meeting time"</label>
                            <TimeField
                                value=MaybeProp::derive(move || time.get())
                                on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
                            >
                                <TimeFieldInput
                                    attr:class="inline-flex items-center rounded-lg border border-slate-300 px-3 py-2 text-sm font-mono text-slate-700 focus-within:border-accent-500 focus-within:ring-1 focus-within:ring-accent-500 [&_[data-pith-time-segment]]:outline-none [&_[data-pith-time-segment]:focus]:bg-accent-100 [&_[data-pith-time-segment]:focus]:text-accent-700 [&_[data-pith-time-segment]]:rounded [&_[data-pith-time-segment]]:px-0.5"
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
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::time_field::*;

let (time, set_time) = signal::<Option<NaiveTime>>(None);

view! {
    <TimeField
        value=MaybeProp::derive(move || time.get())
        on_value_change=Callback::new(move |t: NaiveTime| set_time.set(Some(t)))
    >
        <TimeFieldInput aria_label="Meeting time" />
    </TimeField>
}

// 24-hour format with seconds:
view! {
    <TimeField hour_cycle=HourCycle::H24 granularity=TimeGranularity::Second>
        <TimeFieldInput aria_label="Time" />
    </TimeField>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="TimeField" description="Root component. Manages time value and segment focus." />
                    <PartItem name="TimeFieldInput" description="Renders the segmented time input with spinbutton behavior." />
                </div>
            </DocSection>
        </div>
    }
}
