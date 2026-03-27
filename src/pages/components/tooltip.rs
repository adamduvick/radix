use leptos::prelude::*;
use pith_ui::tooltip::*;

use crate::components::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Tooltip"
                description="A popup that displays information related to an element on hover or keyboard focus."
                features=&["Accessible", "Delayed open", "Keyboard support", "Collision-aware positioning"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="flex gap-4">
                        <TooltipProvider>
                            <Tooltip>
                                <TooltipTrigger attr:class="rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors">
                                    "Hover me"
                                </TooltipTrigger>
                                <TooltipPortal>
                                    <TooltipContent
                                        attr:class="tooltip-content rounded-md bg-slate-900 px-3 py-1.5 text-xs text-white shadow-md"
                                        side_offset=5.0
                                    >
                                        "This is a tooltip"
                                        <TooltipArrow attr:class="fill-slate-900" />
                                    </TooltipContent>
                                </TooltipPortal>
                            </Tooltip>

                            <Tooltip>
                                <TooltipTrigger attr:class="rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-accent-700 transition-colors">
                                    "Save"
                                </TooltipTrigger>
                                <TooltipPortal>
                                    <TooltipContent
                                        attr:class="tooltip-content rounded-md bg-slate-900 px-3 py-1.5 text-xs text-white shadow-md"
                                        side_offset=5.0
                                    >
                                        "Save your changes"
                                        <TooltipArrow attr:class="fill-slate-900" />
                                    </TooltipContent>
                                </TooltipPortal>
                            </Tooltip>
                        </TooltipProvider>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::tooltip::*;

view! {
    <TooltipProvider>
        <Tooltip>
            <TooltipTrigger>"Hover me"</TooltipTrigger>
            <TooltipPortal>
                <TooltipContent side_offset=5.0>
                    "Tooltip text"
                    <TooltipArrow />
                </TooltipContent>
            </TooltipPortal>
        </Tooltip>
    </TooltipProvider>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="TooltipProvider" description="Wraps your app to provide shared tooltip delay behavior." />
                    <PartItem name="Tooltip" description="Root component. Manages open/close state." />
                    <PartItem name="TooltipTrigger" description="The element that triggers the tooltip on hover/focus." />
                    <PartItem name="TooltipPortal" description="Portals the tooltip content to the document body." />
                    <PartItem name="TooltipContent" description="The floating tooltip content." />
                    <PartItem name="TooltipArrow" description="An optional arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
