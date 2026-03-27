use leptos::prelude::*;
use pith_ui::toggle::Toggle;

use crate::components::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    let (bold, set_bold) = signal(false);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toggle"
                description="A two-state button that can be switched on or off."
                features=&["Accessible", "Keyboard toggle", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="flex items-center gap-2">
                        <Toggle
                            pressed=bold
                            on_pressed_change=move |v| set_bold.set(v)
                            attr:class="inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            attr:aria-label="Toggle bold"
                        >
                            <span class="font-bold">"B"</span>
                        </Toggle>
                        <Toggle
                            attr:class="inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            attr:aria-label="Toggle italic"
                        >
                            <span class="italic">"I"</span>
                        </Toggle>
                        <Toggle
                            attr:class="inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            attr:aria-label="Toggle underline"
                        >
                            <span class="underline">"U"</span>
                        </Toggle>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::toggle::Toggle;

let (pressed, set_pressed) = signal(false);

view! {
    <Toggle
        pressed=pressed
        on_pressed_change=move |v| set_pressed.set(v)
        attr:aria-label="Toggle bold"
    >
        "B"
    </Toggle>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Toggle" description="A button that can be toggled on or off. Exposes data-state='on'|'off'." />
                </div>
            </DocSection>
        </div>
    }
}
