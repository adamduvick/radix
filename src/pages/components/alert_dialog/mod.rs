use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::AlertDialogBasicSection;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Alert Dialog"
                description="A modal dialog that interrupts the user with important content and expects a response."
                features=&["Accessible", "Focus trapping", "No outside dismiss", "Action & cancel buttons"]
            />

            <DocSection title="Demo">
                <AlertDialogBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="AlertDialog" description="Root component. Manages open state." />
                    <PartItem name="AlertDialogTrigger" description="Button that opens the alert dialog." />
                    <PartItem name="AlertDialogPortal" description="Portals content to the document body." />
                    <PartItem name="AlertDialogOverlay" description="Background overlay. Cannot be dismissed by clicking." />
                    <PartItem name="AlertDialogContent" description="The dialog panel. Cannot be dismissed with Escape." />
                    <PartItem name="AlertDialogTitle" description="Accessible title." />
                    <PartItem name="AlertDialogDescription" description="Accessible description." />
                    <PartItem name="AlertDialogAction" description="Confirms the action and closes the dialog." />
                    <PartItem name="AlertDialogCancel" description="Cancels and closes the dialog." />
                </div>
            </DocSection>
        </div>
    }
}
