use leptos::prelude::*;
use pith_ui::label::Label;

use crate::components::{extract_demo, DemoTabs};

const LABEL: &str = "block text-sm font-medium text-slate-700 mb-1";
const LABEL_INLINE: &str = "flex items-center gap-2 text-sm text-slate-700 cursor-pointer select-none";
const INPUT: &str = "w-full rounded-md border border-slate-300 px-3 py-2 text-sm \
    focus:border-accent-500 focus:outline-none focus:ring-1 focus:ring-accent-500";

// #region demo
#[component]
pub fn LabelBasic() -> impl IntoView {
    view! {
        <div class="space-y-4 max-w-sm">
            <div>
                <Label attr:r#for="email" attr:class=LABEL>
                    "Email address"
                </Label>
                <input
                    id="email"
                    type="email"
                    placeholder="you@example.com"
                    class=INPUT
                />
            </div>
            <div>
                <Label attr:class=LABEL_INLINE>
                    <input type="checkbox" class="rounded border-slate-300" />
                    "Wrapped label (click anywhere)"
                </Label>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn LabelBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A label linked to an input via the for attribute, and a wrapping label that associates by nesting."
        </p>

        <DemoTabs source=extract_demo(include_str!("label_basic.rs"))>
            <LabelBasic />
        </DemoTabs>
    }
}
