use leptos::prelude::*;
use pith_ui::collapsible::*;

use crate::components::{extract_demo, DemoTabs};

const ROOT: &str = "w-full max-w-sm";
const TRIGGER: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md \
    text-slate-500 hover:bg-slate-100 transition-colors";
const CONTENT: &str = "collapsible-content overflow-hidden";
const REPO_ITEM: &str = "rounded-md border border-slate-200 px-4 py-2.5 text-sm text-slate-700";

// #region demo
#[component]
pub fn CollapsibleBasic() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Collapsible
            open=open
            on_open_change=Callback::new(move |v: bool| set_open.set(v))
            attr:class=ROOT
        >
            <div class="flex items-center justify-between">
                <span class="text-sm font-semibold text-slate-900">"3 repositories starred"</span>
                <CollapsibleTrigger attr:class=TRIGGER>
                    {move || if open.get() { "\u{2212}" } else { "+" }}
                </CollapsibleTrigger>
            </div>

            <div class="mt-2 rounded-md border border-slate-200 px-4 py-2.5 text-sm text-slate-700">
                "pith-ui/pith-ui"
            </div>

            <CollapsibleContent attr:class=CONTENT>
                <div class="mt-2 space-y-2">
                    <div class=REPO_ITEM>
                        "leptos-rs/leptos"
                    </div>
                    <div class=REPO_ITEM>
                        "radix-ui/primitives"
                    </div>
                </div>
            </CollapsibleContent>
        </Collapsible>
    }
}
// #endregion demo

#[component]
pub fn CollapsibleBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A controlled collapsible that reveals additional list items when toggled."
        </p>

        <DemoTabs source=extract_demo(include_str!("collapsible_basic.rs"))>
            <CollapsibleBasic />
        </DemoTabs>
    }
}
