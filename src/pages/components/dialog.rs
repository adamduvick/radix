use leptos::prelude::*;
use pith_ui::dialog::*;

use crate::components::*;

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
                <DemoSection>
                    <Dialog>
                        <DialogTrigger attr:class="rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white hover:bg-accent-700 transition-colors">
                            "Open Dialog"
                        </DialogTrigger>
                        <DialogPortal>
                            <DialogOverlay attr:class="dialog-overlay fixed inset-0 bg-black/50" />
                            <DialogContent attr:class="dialog-content fixed left-1/2 top-1/2 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white p-6 shadow-xl">
                                <DialogTitle attr:class="text-lg font-semibold text-slate-900">
                                    "Edit Profile"
                                </DialogTitle>
                                <DialogDescription attr:class="mt-2 text-sm text-slate-600">
                                    "Make changes to your profile here. Click save when you're done."
                                </DialogDescription>
                                <div class="mt-6 space-y-4">
                                    <div>
                                        <label class="block text-sm font-medium text-slate-700">"Name"</label>
                                        <input
                                            type="text"
                                            value="Ada Lovelace"
                                            class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-slate-700">"Email"</label>
                                        <input
                                            type="email"
                                            value="ada@example.com"
                                            class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500"
                                        />
                                    </div>
                                </div>
                                <div class="mt-6 flex justify-end gap-3">
                                    <DialogClose attr:class="rounded-lg border border-slate-300 px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 transition-colors">
                                        "Cancel"
                                    </DialogClose>
                                    <DialogClose attr:class="rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white hover:bg-accent-700 transition-colors">
                                        "Save changes"
                                    </DialogClose>
                                </div>
                            </DialogContent>
                        </DialogPortal>
                    </Dialog>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::dialog::*;

view! {
    <Dialog>
        <DialogTrigger>"Open"</DialogTrigger>
        <DialogPortal>
            <DialogOverlay attr:class="overlay" />
            <DialogContent attr:class="modal">
                <DialogTitle>"Title"</DialogTitle>
                <DialogDescription>"Description"</DialogDescription>
                <DialogClose>"Close"</DialogClose>
            </DialogContent>
        </DialogPortal>
    </Dialog>
}"# />
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
