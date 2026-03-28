use leptos::prelude::*;
use pith_ui::separator::*;

use crate::components::{extract_demo, DemoTabs};

const HORIZONTAL: &str = "my-4 h-px bg-slate-200";
const VERTICAL: &str = "h-full w-px bg-slate-200";

// #region demo
#[component]
pub fn SeparatorBasic() -> impl IntoView {
    view! {
        <div class="max-w-sm">
            <div>
                <h3 class="text-base font-semibold text-slate-900">"Pith UI"</h3>
                <p class="text-sm text-slate-600">"An open-source component library."</p>
            </div>
            <Separator attr:class=HORIZONTAL decorative=true />
            <div class="flex h-5 items-center gap-4 text-sm text-slate-600">
                <span>"Blog"</span>
                <Separator orientation=Orientation::Vertical attr:class=VERTICAL decorative=true />
                <span>"Docs"</span>
                <Separator orientation=Orientation::Vertical attr:class=VERTICAL decorative=true />
                <span>"Source"</span>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn SeparatorBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "Horizontal and vertical separators used to divide content sections."
        </p>

        <DemoTabs source=extract_demo(include_str!("separator_basic.rs"))>
            <SeparatorBasic />
        </DemoTabs>
    }
}
