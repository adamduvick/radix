use leptos::prelude::*;
use pith_ui::alert_dialog::*;

use crate::components::*;

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
                <DemoSection>
                    <AlertDialog>
                        <AlertDialogTrigger attr:class="rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 transition-colors">
                            "Delete Account"
                        </AlertDialogTrigger>
                        <AlertDialogPortal>
                            <AlertDialogOverlay attr:class="dialog-overlay fixed inset-0 bg-black/50" />
                            <AlertDialogContent attr:class="dialog-content fixed left-1/2 top-1/2 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white p-6 shadow-xl">
                                <AlertDialogTitle attr:class="text-lg font-semibold text-slate-900">
                                    "Are you sure?"
                                </AlertDialogTitle>
                                <AlertDialogDescription attr:class="mt-2 text-sm text-slate-600">
                                    "This action cannot be undone. This will permanently delete your account and remove all your data from our servers."
                                </AlertDialogDescription>
                                <div class="mt-6 flex justify-end gap-3">
                                    <AlertDialogCancel attr:class="rounded-lg border border-slate-300 px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors">
                                        "Cancel"
                                    </AlertDialogCancel>
                                    <AlertDialogAction attr:class="rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 transition-colors">
                                        "Delete"
                                    </AlertDialogAction>
                                </div>
                            </AlertDialogContent>
                        </AlertDialogPortal>
                    </AlertDialog>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::alert_dialog::*;

view! {
    <AlertDialog>
        <AlertDialogTrigger>"Delete"</AlertDialogTrigger>
        <AlertDialogPortal>
            <AlertDialogOverlay attr:class="overlay" />
            <AlertDialogContent attr:class="modal">
                <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                <AlertDialogDescription>
                    "This action cannot be undone."
                </AlertDialogDescription>
                <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                <AlertDialogAction>"Confirm"</AlertDialogAction>
            </AlertDialogContent>
        </AlertDialogPortal>
    </AlertDialog>
}"# />
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
