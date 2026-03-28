use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::DialogBasicSection;

#[component]
pub fn DialogPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Dialog"
                description="A modal or non-modal overlay window that focuses the user's attention."
                features=&["Accessible", "Focus trapping", "Keyboard dismiss", "Modal & non-modal", "Animated"]
            />

            <DocSection title="Demo">
                <DialogBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Dialog" description="Root component. Manages open state." />
                    <PartItem name="DialogTrigger" description="Button that opens the dialog." />
                    <PartItem name="DialogPortal" description="Portals content to document body." />
                    <PartItem name="DialogOverlay" description="Background overlay behind the dialog." />
                    <PartItem name="DialogContent" description="The dialog panel with focus trapping." />
                    <PartItem name="DialogTitle" description="Accessible title for the dialog." />
                    <PartItem name="DialogDescription" description="Accessible description." />
                    <PartItem name="DialogClose" description="Button that closes the dialog." />
                </div>
            </DocSection>
        </div>
    }
}
