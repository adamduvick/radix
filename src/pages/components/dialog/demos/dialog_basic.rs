use leptos::prelude::*;
use pith_ui::dialog::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER_BTN: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium \
    text-white hover:bg-accent-700 transition-colors";
const OVERLAY: &str = "dialog-overlay fixed inset-0 bg-black/50";
const CONTENT: &str = "dialog-content fixed left-1/2 top-1/2 w-full max-w-md \
    -translate-x-1/2 -translate-y-1/2 rounded-xl bg-white p-6 shadow-xl";
const TITLE: &str = "text-lg font-semibold text-slate-900";
const DESCRIPTION: &str = "mt-2 text-sm text-slate-600";
const LABEL: &str = "block text-sm font-medium text-slate-700";
const INPUT: &str = "mt-1 w-full rounded-md border border-slate-300 px-3 py-2 \
    text-sm focus:border-accent-500 focus:outline-none focus:ring-1 \
    focus:ring-accent-500";
const CANCEL_BTN: &str = "rounded-lg border border-slate-300 px-4 py-2 text-sm \
    font-medium text-slate-700 hover:bg-slate-50 transition-colors";
const SAVE_BTN: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium \
    text-white hover:bg-accent-700 transition-colors";

// #region demo
#[component]
pub fn DialogBasic() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class=TRIGGER_BTN>
                "Open Dialog"
            </DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=OVERLAY />
                <DialogContent attr:class=CONTENT>
                    <DialogTitle attr:class=TITLE>
                        "Edit Profile"
                    </DialogTitle>
                    <DialogDescription attr:class=DESCRIPTION>
                        "Make changes to your profile here. Click save when you're done."
                    </DialogDescription>
                    <div class="mt-6 space-y-4">
                        <div>
                            <label class=LABEL>"Name"</label>
                            <input
                                type="text"
                                value="Ada Lovelace"
                                class=INPUT
                            />
                        </div>
                        <div>
                            <label class=LABEL>"Email"</label>
                            <input
                                type="email"
                                value="ada@example.com"
                                class=INPUT
                            />
                        </div>
                    </div>
                    <div class="mt-6 flex justify-end gap-3">
                        <DialogClose attr:class=CANCEL_BTN>
                            "Cancel"
                        </DialogClose>
                        <DialogClose attr:class=SAVE_BTN>
                            "Save changes"
                        </DialogClose>
                    </div>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
// #endregion demo

#[component]
pub fn DialogBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A modal dialog with a form for editing profile information, demonstrating overlay, focus trapping, and close buttons."
        </p>

        <DemoTabs source=extract_demo(include_str!("dialog_basic.rs"))>
            <DialogBasic />
        </DemoTabs>
    }
}
