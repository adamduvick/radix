use leptos::prelude::*;
use pith_ui::context_menu::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "flex h-36 w-full items-center justify-center rounded-lg \
    border-2 border-dashed border-slate-300 text-sm text-slate-500 select-none";
const CONTENT: &str = "dropdown-content min-w-[180px] rounded-lg border \
    border-slate-200 bg-white p-1 shadow-lg";
const ITEM: &str = "flex cursor-pointer select-none items-center rounded-md px-2 \
    py-1.5 text-sm text-slate-700 outline-none \
    data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700";
const SEPARATOR: &str = "mx-1 my-1 h-px bg-slate-200";

// #region demo
#[component]
pub fn ContextMenuBasic() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger attr:class=TRIGGER>
                "Right-click here"
            </ContextMenuTrigger>
            <ContextMenuPortal>
                <ContextMenuContent attr:class=CONTENT>
                    <ContextMenuItem attr:class=ITEM>"Back"</ContextMenuItem>
                    <ContextMenuItem attr:class=ITEM>"Forward"</ContextMenuItem>
                    <ContextMenuItem attr:class=ITEM>"Reload"</ContextMenuItem>
                    <ContextMenuSeparator attr:class=SEPARATOR />
                    <ContextMenuItem attr:class=ITEM>"View Source"</ContextMenuItem>
                    <ContextMenuItem attr:class=ITEM>"Inspect"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenuPortal>
        </ContextMenu>
    }
}
// #endregion demo

#[component]
pub fn ContextMenuBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A context menu triggered by right-clicking on a designated area, with browser-style navigation items."
        </p>

        <DemoTabs source=extract_demo(include_str!("context_menu_basic.rs"))>
            <ContextMenuBasic />
        </DemoTabs>
    }
}
