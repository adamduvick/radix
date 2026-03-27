use leptos::prelude::*;
use pith_ui::avatar::*;

use crate::components::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Avatar"
                description="An image element with a fallback for representing the user."
                features=&["Accessible", "Fallback support", "Loading states", "Delayed fallback"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="flex items-center gap-4">
                        <Avatar attr:class="relative flex h-12 w-12 shrink-0 overflow-hidden rounded-full">
                            <AvatarFallback
                                attr:class="flex h-full w-full items-center justify-center rounded-full bg-accent-100 text-sm font-medium text-accent-700"
                            >
                                "AL"
                            </AvatarFallback>
                        </Avatar>

                        <Avatar attr:class="relative flex h-12 w-12 shrink-0 overflow-hidden rounded-full">
                            <AvatarFallback
                                attr:class="flex h-full w-full items-center justify-center rounded-full bg-emerald-100 text-sm font-medium text-emerald-700"
                            >
                                "JD"
                            </AvatarFallback>
                        </Avatar>

                        <Avatar attr:class="relative flex h-12 w-12 shrink-0 overflow-hidden rounded-full">
                            <AvatarFallback
                                attr:class="flex h-full w-full items-center justify-center rounded-full bg-amber-100 text-sm font-medium text-amber-700"
                            >
                                "MK"
                            </AvatarFallback>
                        </Avatar>

                        <Avatar attr:class="relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full">
                            <AvatarFallback
                                attr:class="flex h-full w-full items-center justify-center rounded-full bg-slate-200 text-xs font-medium text-slate-600"
                            >
                                "+3"
                            </AvatarFallback>
                        </Avatar>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::avatar::*;

view! {
    <Avatar attr:class="avatar">
        <AvatarImage src="photo.jpg" attr:alt="User" />
        <AvatarFallback delay_ms=300>
            "AB"
        </AvatarFallback>
    </Avatar>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Avatar" description="Root container for the avatar." />
                    <PartItem name="AvatarImage" description="The image element. Handles loading and error states." />
                    <PartItem name="AvatarFallback" description="Renders when no image or while loading. Supports a delay." />
                </div>
            </DocSection>
        </div>
    }
}
