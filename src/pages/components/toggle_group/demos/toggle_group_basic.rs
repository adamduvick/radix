use leptos::prelude::*;
use pith_ui::toggle_group::*;

use crate::components::{extract_demo, DemoTabs};

const GROUP: &str = "inline-flex rounded-lg border border-slate-200 bg-slate-50 p-0.5";
const GROUP_MULTIPLE: &str = "inline-flex gap-0.5 rounded-lg border border-slate-200 bg-slate-50 p-0.5";
const ITEM: &str = "rounded-md px-3 py-1.5 text-sm font-medium text-slate-600 \
    transition-colors data-[state=on]:bg-white data-[state=on]:text-slate-900 \
    data-[state=on]:shadow-sm";
const ITEM_ACCENT: &str = "rounded-md px-3 py-1.5 text-sm text-slate-600 \
    transition-colors data-[state=on]:bg-white data-[state=on]:text-accent-700 \
    data-[state=on]:shadow-sm";

// #region demo
#[component]
pub fn ToggleGroupBasic() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <label class="mb-2 block text-sm font-medium text-slate-700">"Text alignment (single)"</label>
                <ToggleGroupSingle
                    default_value="center".to_string()
                    attr:class=GROUP
                >
                    <ToggleGroupItem
                        value="left".to_string()
                        attr:class=ITEM
                    >
                        "Left"
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="center".to_string()
                        attr:class=ITEM
                    >
                        "Center"
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="right".to_string()
                        attr:class=ITEM
                    >
                        "Right"
                    </ToggleGroupItem>
                </ToggleGroupSingle>
            </div>

            <div>
                <label class="mb-2 block text-sm font-medium text-slate-700">"Formatting (multiple)"</label>
                <ToggleGroupMultiple
                    attr:class=GROUP_MULTIPLE
                >
                    <ToggleGroupItem
                        value="bold".to_string()
                        attr:class=ITEM_ACCENT
                    >
                        <span class="font-bold">"B"</span>
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="italic".to_string()
                        attr:class=ITEM_ACCENT
                    >
                        <span class="italic">"I"</span>
                    </ToggleGroupItem>
                    <ToggleGroupItem
                        value="underline".to_string()
                        attr:class=ITEM_ACCENT
                    >
                        <span class="underline">"U"</span>
                    </ToggleGroupItem>
                </ToggleGroupMultiple>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn ToggleGroupBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "Single-select alignment group and multi-select formatting group demonstrating both toggle group modes."
        </p>

        <DemoTabs source=extract_demo(include_str!("toggle_group_basic.rs"))>
            <ToggleGroupBasic />
        </DemoTabs>
    }
}
