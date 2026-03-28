use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::AvatarBasicSection;

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
                <AvatarBasicSection />
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
