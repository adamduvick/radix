use leptos::prelude::*;
use pith_ui::hover_card::*;

use crate::components::*;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Hover Card"
                description="Displays rich content when hovering over a trigger, like a user profile preview."
                features=&["Accessible", "Open/close delay", "Collision-aware", "Text selection support"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="flex gap-4">
                        <HoverCard>
                            <HoverCardTrigger attr:class="inline-flex items-center rounded-full bg-accent-50 px-3 py-1 text-sm font-medium text-accent-700 underline decoration-accent-300 hover:bg-accent-100 transition-colors cursor-pointer">
                                "@pith-ui"
                            </HoverCardTrigger>
                            <HoverCardPortal>
                                <HoverCardContent
                                    attr:class="popover-content w-64 rounded-xl border border-slate-200 bg-white p-4 shadow-lg"
                                    side_offset=5.0
                                >
                                    <div class="space-y-2">
                                        <div class="flex h-10 w-10 items-center justify-center rounded-full bg-accent-100 text-sm font-bold text-accent-700">
                                            "P"
                                        </div>
                                        <div>
                                            <h4 class="text-sm font-semibold text-slate-900">"Pith UI"</h4>
                                            <p class="text-xs text-slate-500">"@pith-ui"</p>
                                        </div>
                                        <p class="text-sm text-slate-600">
                                            "Accessible UI primitives for Leptos. The core of your interface."
                                        </p>
                                    </div>
                                    <HoverCardArrow attr:class="fill-white [filter:drop-shadow(0_1px_2px_rgb(0_0_0/0.06))]" />
                                </HoverCardContent>
                            </HoverCardPortal>
                        </HoverCard>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::hover_card::*;

view! {
    <HoverCard>
        <HoverCardTrigger>"@user"</HoverCardTrigger>
        <HoverCardPortal>
            <HoverCardContent side_offset=5.0>
                "Preview content"
                <HoverCardArrow />
            </HoverCardContent>
        </HoverCardPortal>
    </HoverCard>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="HoverCard" description="Root component. Manages open/close state with delay." />
                    <PartItem name="HoverCardTrigger" description="The element that triggers the hover card." />
                    <PartItem name="HoverCardPortal" description="Portals content to the body." />
                    <PartItem name="HoverCardContent" description="The floating content panel." />
                    <PartItem name="HoverCardArrow" description="An arrow pointing to the trigger." />
                </div>
            </DocSection>
        </div>
    }
}
