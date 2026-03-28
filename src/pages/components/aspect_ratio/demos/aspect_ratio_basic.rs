use leptos::prelude::*;
use pith_ui::aspect_ratio::AspectRatio;

use crate::components::{extract_demo, DemoTabs};

const RATIO_LABEL: &str = "mb-2 text-xs font-medium text-slate-500";
const RATIO_16_9: &str = "flex h-full w-full items-center justify-center rounded-lg \
    bg-accent-100 text-sm font-medium text-accent-700";
const RATIO_1_1: &str = "flex h-full w-full items-center justify-center rounded-lg \
    bg-emerald-100 text-sm font-medium text-emerald-700";
const RATIO_4_3: &str = "flex h-full w-full items-center justify-center rounded-lg \
    bg-amber-100 text-sm font-medium text-amber-700";

// #region demo
#[component]
pub fn AspectRatioBasic() -> impl IntoView {
    view! {
        <div class="grid gap-4 sm:grid-cols-3 max-w-lg">
            <div>
                <p class=RATIO_LABEL>"16:9"</p>
                <AspectRatio ratio=16.0 / 9.0>
                    <div class=RATIO_16_9>"16:9"</div>
                </AspectRatio>
            </div>
            <div>
                <p class=RATIO_LABEL>"1:1"</p>
                <AspectRatio ratio=1.0>
                    <div class=RATIO_1_1>"1:1"</div>
                </AspectRatio>
            </div>
            <div>
                <p class=RATIO_LABEL>"4:3"</p>
                <AspectRatio ratio=4.0 / 3.0>
                    <div class=RATIO_4_3>"4:3"</div>
                </AspectRatio>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn AspectRatioBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "Three common aspect ratios side by side: 16:9, 1:1, and 4:3."
        </p>

        <DemoTabs source=extract_demo(include_str!("aspect_ratio_basic.rs"))>
            <AspectRatioBasic />
        </DemoTabs>
    }
}
