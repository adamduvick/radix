use leptos::prelude::*;
use pith_ui::radio_group::*;

use crate::components::{extract_demo, DemoTabs};

const ITEM: &str = "flex h-5 w-5 items-center justify-center rounded-full border \
    border-slate-300 bg-white transition-colors \
    data-[state=checked]:border-accent-600";
const DOT: &str = "block h-2.5 w-2.5 rounded-full bg-accent-600";

// #region demo
#[component]
pub fn RadioGroupBasic() -> impl IntoView {
    view! {
        <RadioGroup default_value="comfortable".to_string() attr:class="space-y-3 max-w-sm">
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <RadioGroupItem value="default".to_string() attr:class=ITEM>
                    <RadioGroupIndicator attr:class=DOT />
                </RadioGroupItem>
                <span class="text-sm text-slate-700">"Default"</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <RadioGroupItem value="comfortable".to_string() attr:class=ITEM>
                    <RadioGroupIndicator attr:class=DOT />
                </RadioGroupItem>
                <span class="text-sm text-slate-700">"Comfortable"</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer select-none">
                <RadioGroupItem value="compact".to_string() attr:class=ITEM>
                    <RadioGroupIndicator attr:class=DOT />
                </RadioGroupItem>
                <span class="text-sm text-slate-700">"Compact"</span>
            </label>
        </RadioGroup>
    }
}
// #endregion demo

#[component]
pub fn RadioGroupBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A radio group with three mutually exclusive options."
        </p>

        <DemoTabs source=extract_demo(include_str!("radio_group_basic.rs"))>
            <RadioGroupBasic />
        </DemoTabs>
    }
}
