use leptos::prelude::*;
use pith_ui::alert_dialog::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white \
    hover:bg-red-700 transition-colors";
const OVERLAY: &str = "dialog-overlay fixed inset-0 bg-black/50";
const CONTENT: &str = "dialog-content fixed left-1/2 top-1/2 w-full max-w-md -translate-x-1/2 \
    -translate-y-1/2 rounded-xl bg-white p-6 shadow-xl";
const TITLE: &str = "text-lg font-semibold text-slate-900";
const DESCRIPTION: &str = "mt-2 text-sm text-slate-600";
const CANCEL: &str = "rounded-lg border border-slate-300 px-4 py-2 text-sm font-medium \
    text-slate-700 hover:bg-slate-50 transition-colors";
const ACTION: &str = "rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white \
    hover:bg-red-700 transition-colors";

// #region demo
#[component]
pub fn AlertDialogBasic() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger attr:class=TRIGGER>
                "Delete Account"
            </AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay attr:class=OVERLAY />
                <AlertDialogContent attr:class=CONTENT>
                    <AlertDialogTitle attr:class=TITLE>
                        "Are you sure?"
                    </AlertDialogTitle>
                    <AlertDialogDescription attr:class=DESCRIPTION>
                        "This action cannot be undone. This will permanently delete your account and remove all your data from our servers."
                    </AlertDialogDescription>
                    <div class="mt-6 flex justify-end gap-3">
                        <AlertDialogCancel attr:class=CANCEL>
                            "Cancel"
                        </AlertDialogCancel>
                        <AlertDialogAction attr:class=ACTION>
                            "Delete"
                        </AlertDialogAction>
                    </div>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>
    }
}
// #endregion demo

#[component]
pub fn AlertDialogBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A destructive confirmation dialog with cancel and action buttons."
        </p>

        <DemoTabs source=extract_demo(include_str!("alert_dialog_basic.rs"))>
            <AlertDialogBasic />
        </DemoTabs>
    }
}
