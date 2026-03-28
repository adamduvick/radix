use leptos::prelude::*;
use pith_ui::popover::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm \
    font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors";
const CONTENT: &str = "popover-content w-72 rounded-xl border border-slate-200 \
    bg-white p-4 shadow-lg";
const INPUT: &str = "flex-1 rounded-md border border-slate-200 px-2.5 py-1 text-sm";
const ARROW: &str = "fill-white [filter:drop-shadow(0_1px_2px_rgb(0_0_0/0.06))]";

// #region demo
#[component]
pub fn PopoverBasic() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger attr:class=TRIGGER>
                "Open popover"
            </PopoverTrigger>
            <PopoverPortal>
                <PopoverContent attr:class=CONTENT side_offset=5.0>
                    <div class="space-y-3">
                        <h3 class="text-sm font-semibold text-slate-900">"Dimensions"</h3>
                        <div class="space-y-2">
                            <div class="flex items-center gap-3">
                                <label class="w-20 text-xs text-slate-600">"Width"</label>
                                <input type="text" value="100%" class=INPUT />
                            </div>
                            <div class="flex items-center gap-3">
                                <label class="w-20 text-xs text-slate-600">"Height"</label>
                                <input type="text" value="auto" class=INPUT />
                            </div>
                            <div class="flex items-center gap-3">
                                <label class="w-20 text-xs text-slate-600">"Max width"</label>
                                <input type="text" value="300px" class=INPUT />
                            </div>
                        </div>
                    </div>
                    <PopoverArrow attr:class=ARROW />
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
// #endregion demo

#[component]
pub fn PopoverBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A popover with a form layout, anchored to a trigger button with an arrow."
        </p>

        <DemoTabs source=extract_demo(include_str!("popover_basic.rs"))>
            <PopoverBasic />
        </DemoTabs>
    }
}
