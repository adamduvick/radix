use leptos::prelude::*;
use pith_ui::toast::*;

use crate::components::{extract_demo, DemoTabs};

const BUTTON: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium text-white \
    hover:bg-accent-700 transition-colors";
const TOAST: &str = "rounded-lg border border-slate-200 bg-white p-4 shadow-lg";
const TITLE: &str = "text-sm font-semibold text-slate-900";
const DESCRIPTION: &str = "mt-1 text-sm text-slate-600";
const CLOSE: &str = "absolute right-2 top-2 rounded-md p-1 text-slate-400 \
    hover:text-slate-600";

// #region demo
#[component]
pub fn ToastBasic() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <ToastProvider>
            <button
                class=BUTTON
                on:click=move |_| set_open.set(true)
            >
                "Show Toast"
            </button>
            <Toast
                open=open
                on_open_change=Callback::new(move |v: bool| set_open.set(v))
                attr:class=TOAST
            >
                <ToastTitle attr:class=TITLE>
                    "Changes saved"
                </ToastTitle>
                <ToastDescription attr:class=DESCRIPTION>
                    "Your profile has been updated successfully."
                </ToastDescription>
                <ToastClose attr:class=CLOSE>
                    "\u{2715}"
                </ToastClose>
            </Toast>
            <ToastViewport attr:class="toast-viewport" />
        </ToastProvider>
    }
}
// #endregion demo

#[component]
pub fn ToastBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A toast notification triggered by a button, with auto-dismiss and a close button."
        </p>

        <DemoTabs source=extract_demo(include_str!("toast_basic.rs"))>
            <ToastBasic />
        </DemoTabs>
    }
}
