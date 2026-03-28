use leptos::prelude::*;
use pith_ui::tooltip::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "rounded-lg border border-slate-200 bg-white px-4 py-2 \
    text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors";
const TRIGGER_ACCENT: &str = "rounded-lg bg-accent-600 px-4 py-2 text-sm font-medium \
    text-white shadow-sm hover:bg-accent-700 transition-colors";
const CONTENT: &str = "tooltip-content rounded-md bg-slate-900 px-3 py-1.5 text-xs \
    text-white shadow-md";
const ARROW: &str = "fill-slate-900";

// #region demo
#[component]
pub fn TooltipBasic() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger attr:class=TRIGGER>
                        "Hover me"
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent
                            attr:class=CONTENT
                            side_offset=5.0
                        >
                            "This is a tooltip"
                            <TooltipArrow attr:class=ARROW />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>

                <Tooltip>
                    <TooltipTrigger attr:class=TRIGGER_ACCENT>
                        "Save"
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent
                            attr:class=CONTENT
                            side_offset=5.0
                        >
                            "Save your changes"
                            <TooltipArrow attr:class=ARROW />
                        </TooltipContent>
                    </TooltipPortal>
                </Tooltip>
            </TooltipProvider>
        </div>
    }
}
// #endregion demo

#[component]
pub fn TooltipBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "Tooltips on hover and focus, with a shared provider for coordinated open delays."
        </p>

        <DemoTabs source=extract_demo(include_str!("tooltip_basic.rs"))>
            <TooltipBasic />
        </DemoTabs>
    }
}
