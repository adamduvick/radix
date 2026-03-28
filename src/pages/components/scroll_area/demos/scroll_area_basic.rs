use leptos::prelude::*;
use pith_ui::scroll_area::*;

use crate::components::{extract_demo, DemoTabs};

const SCROLL_AREA: &str = "h-48 w-full max-w-md rounded-lg border border-slate-200";
const SCROLLBAR: &str = "flex w-2.5 touch-none select-none bg-transparent p-0.5 \
    transition-colors";
const THUMB: &str = "relative flex-1 rounded-full bg-slate-300 hover:bg-slate-400 \
    transition-colors";
const LIST_ITEM: &str = "rounded-md border border-slate-100 bg-slate-50 px-3 py-2 \
    text-sm text-slate-700";

// #region demo
#[component]
pub fn ScrollAreaBasic() -> impl IntoView {
    view! {
        <ScrollArea attr:class=SCROLL_AREA r#type=ScrollAreaType::Auto>
            <ScrollAreaViewport attr:class="h-full w-full p-4">
                <div class="space-y-4">
                    {(1..=20).map(|i| view! {
                        <div class=LIST_ITEM>
                            {format!("Item {i} — Lorem ipsum dolor sit amet, consectetur adipiscing elit.")}
                        </div>
                    }).collect_view()}
                </div>
            </ScrollAreaViewport>
            <ScrollAreaScrollbar orientation=Orientation::Vertical attr:class=SCROLLBAR>
                <ScrollAreaThumb attr:class=THUMB>" "</ScrollAreaThumb>
            </ScrollAreaScrollbar>
        </ScrollArea>
    }
}
// #endregion demo

#[component]
pub fn ScrollAreaBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A scrollable list with a custom vertical scrollbar that appears automatically."
        </p>

        <DemoTabs source=extract_demo(include_str!("scroll_area_basic.rs"))>
            <ScrollAreaBasic />
        </DemoTabs>
    }
}
