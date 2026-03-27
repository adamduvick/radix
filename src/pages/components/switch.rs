use leptos::prelude::*;
use pith_ui::label::Label;
use pith_ui::switch::*;

use crate::components::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let (airplane, set_airplane) = signal(false);
    let (wifi, set_wifi) = signal(true);

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Switch"
                description="A control that allows the user to toggle between on and off states."
                features=&["Accessible", "Keyboard toggle", "Form integration", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="space-y-6 max-w-sm">
                        <div class="flex items-center justify-between">
                            <Label attr:r#for="airplane" attr:class="text-sm font-medium text-slate-700 select-none">
                                "Airplane Mode"
                            </Label>
                            <Switch
                                attr:id="airplane"
                                checked=airplane
                                on_checked_change=move |v| set_airplane.set(v)
                                attr:class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-slate-200 transition-colors data-[state=checked]:bg-accent-600"
                            >
                                <SwitchThumb attr:class="pointer-events-none block h-5 w-5 translate-x-0 rounded-full bg-white shadow-sm ring-0 transition-transform data-[state=checked]:translate-x-5" />
                            </Switch>
                        </div>

                        <div class="flex items-center justify-between">
                            <Label attr:r#for="wifi" attr:class="text-sm font-medium text-slate-700 select-none">
                                "Wi-Fi"
                            </Label>
                            <Switch
                                attr:id="wifi"
                                checked=wifi
                                on_checked_change=move |v| set_wifi.set(v)
                                attr:class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-slate-200 transition-colors data-[state=checked]:bg-accent-600"
                            >
                                <SwitchThumb attr:class="pointer-events-none block h-5 w-5 translate-x-0 rounded-full bg-white shadow-sm ring-0 transition-transform data-[state=checked]:translate-x-5" />
                            </Switch>
                        </div>

                        <div class="flex items-center justify-between">
                            <Label attr:r#for="disabled-switch" attr:class="text-sm text-slate-400 select-none">
                                "Disabled"
                            </Label>
                            <Switch
                                attr:id="disabled-switch"
                                disabled=true
                                attr:class="relative inline-flex h-6 w-11 shrink-0 cursor-not-allowed rounded-full border-2 border-transparent bg-slate-100 opacity-50"
                            >
                                <SwitchThumb attr:class="pointer-events-none block h-5 w-5 translate-x-0 rounded-full bg-white shadow-sm ring-0" />
                            </Switch>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::switch::*;

let (checked, set_checked) = signal(false);

view! {
    <Switch
        checked=checked
        on_checked_change=move |v| set_checked.set(v)
        attr:class="switch"
    >
        <SwitchThumb attr:class="thumb" />
    </Switch>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Switch" description="The root switch control. Renders a button with a hidden input." />
                    <PartItem name="SwitchThumb" description="The thumb that slides between states." />
                </div>
            </DocSection>
        </div>
    }
}
