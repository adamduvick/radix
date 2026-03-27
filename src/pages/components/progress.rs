use leptos::prelude::*;
use pith_ui::progress::*;

use crate::components::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (value, set_value) = signal(33.0);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Progress"
                description="Displays an indicator showing the completion progress of a task."
                features=&["Accessible", "Determinate & indeterminate", "Custom max value"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-6 max-w-md">
                        <div>
                            <div class="mb-2 flex justify-between text-sm">
                                <span class="font-medium text-slate-700">"Progress"</span>
                                <span class="text-slate-500">{move || format!("{}%", value.get() as u32)}</span>
                            </div>
                            <Progress
                                value=value.get()
                                attr:class="relative h-3 w-full overflow-hidden rounded-full bg-slate-200"
                            >
                                <ProgressIndicator
                                    attr:class="h-full bg-accent-600 transition-all duration-500 ease-out"
                                    attr:style=move || format!("width: {}%", value.get())
                                />
                            </Progress>
                        </div>

                        <div class="flex gap-2">
                            <button
                                class="rounded-md border border-slate-300 px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
                                on:click=move |_| set_value.set(0.0)
                            >
                                "0%"
                            </button>
                            <button
                                class="rounded-md border border-slate-300 px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
                                on:click=move |_| set_value.set(33.0)
                            >
                                "33%"
                            </button>
                            <button
                                class="rounded-md border border-slate-300 px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
                                on:click=move |_| set_value.set(66.0)
                            >
                                "66%"
                            </button>
                            <button
                                class="rounded-md border border-slate-300 px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
                                on:click=move |_| set_value.set(100.0)
                            >
                                "100%"
                            </button>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::progress::*;

view! {
    <Progress value=66.0 attr:class="progress-root">
        <ProgressIndicator
            attr:class="progress-bar"
            attr:style="width: 66%"
        />
    </Progress>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Progress" description="The root progress container. Sets aria attributes." />
                    <PartItem name="ProgressIndicator" description="The visual indicator showing completion." />
                </div>
            </DocSection>
        </div>
    }
}
