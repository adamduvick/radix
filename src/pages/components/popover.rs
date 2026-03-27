use leptos::prelude::*;
use pith_ui::popover::*;

use crate::components::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Popover"
                description="Displays floating content anchored to a trigger element."
                features=&["Accessible", "Focus management", "Collision-aware", "Dismissable", "Arrow support"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <Popover>
                        <PopoverTrigger attr:class="rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors">
                            "Open popover"
                        </PopoverTrigger>
                        <PopoverPortal>
                            <PopoverContent
                                attr:class="popover-content w-72 rounded-xl border border-slate-200 bg-white p-4 shadow-lg"
                                side_offset=5.0
                            >
                                <div class="space-y-3">
                                    <h3 class="text-sm font-semibold text-slate-900">"Dimensions"</h3>
                                    <div class="space-y-2">
                                        <div class="flex items-center gap-3">
                                            <label class="w-20 text-xs text-slate-600">"Width"</label>
                                            <input
                                                type="text"
                                                value="100%"
                                                class="flex-1 rounded-md border border-slate-200 px-2.5 py-1 text-sm"
                                            />
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <label class="w-20 text-xs text-slate-600">"Height"</label>
                                            <input
                                                type="text"
                                                value="auto"
                                                class="flex-1 rounded-md border border-slate-200 px-2.5 py-1 text-sm"
                                            />
                                        </div>
                                        <div class="flex items-center gap-3">
                                            <label class="w-20 text-xs text-slate-600">"Max width"</label>
                                            <input
                                                type="text"
                                                value="300px"
                                                class="flex-1 rounded-md border border-slate-200 px-2.5 py-1 text-sm"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <PopoverArrow attr:class="fill-white [filter:drop-shadow(0_1px_2px_rgb(0_0_0/0.06))]" />
                            </PopoverContent>
                        </PopoverPortal>
                    </Popover>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::popover::*;

view! {
    <Popover>
        <PopoverTrigger>"Open"</PopoverTrigger>
        <PopoverPortal>
            <PopoverContent side_offset=5.0>
                "Popover content"
                <PopoverClose>"Close"</PopoverClose>
                <PopoverArrow />
            </PopoverContent>
        </PopoverPortal>
    </Popover>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Popover" description="Root component. Manages open/close state." />
                    <PartItem name="PopoverTrigger" description="Button that opens the popover." />
                    <PartItem name="PopoverAnchor" description="Optional custom anchor for positioning." />
                    <PartItem name="PopoverPortal" description="Portals content to the body." />
                    <PartItem name="PopoverContent" description="The floating popover panel." />
                    <PartItem name="PopoverClose" description="Button that closes the popover." />
                    <PartItem name="PopoverArrow" description="An arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
