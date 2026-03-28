use leptos::prelude::*;
use pith_ui::dropdown_menu::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "inline-flex items-center gap-1 rounded-lg border \
    border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 \
    shadow-sm hover:bg-slate-50 transition-colors";
const CONTENT: &str = "dropdown-content min-w-[180px] rounded-lg border \
    border-slate-200 bg-white p-1 shadow-lg";
const ITEM: &str = "flex cursor-pointer select-none items-center rounded-md px-2 \
    py-1.5 text-sm text-slate-700 outline-none \
    data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700";
const ITEM_DANGER: &str = "flex cursor-pointer select-none items-center rounded-md \
    px-2 py-1.5 text-sm text-red-600 outline-none \
    data-[highlighted]:bg-red-50";
const SEPARATOR: &str = "mx-1 my-1 h-px bg-slate-200";

fn chevron_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round"
            class="text-slate-400">
            <path d="m4 5 3 3 3-3" />
        </svg>
    }
}

// #region demo
#[component]
pub fn DropdownMenuBasic() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger attr:class=TRIGGER>
                "Options"
                {chevron_icon()}
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
                <DropdownMenuContent
                    attr:class=CONTENT
                    side_offset=4.0
                >
                    <DropdownMenuItem attr:class=ITEM>"New File"</DropdownMenuItem>
                    <DropdownMenuItem attr:class=ITEM>"Open..."</DropdownMenuItem>
                    <DropdownMenuSeparator attr:class=SEPARATOR />
                    <DropdownMenuItem attr:class=ITEM>"Save"</DropdownMenuItem>
                    <DropdownMenuItem attr:class=ITEM>"Save As..."</DropdownMenuItem>
                    <DropdownMenuSeparator attr:class=SEPARATOR />
                    <DropdownMenuItem attr:class=ITEM_DANGER>
                        "Delete"
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    }
}
// #endregion demo

#[component]
pub fn DropdownMenuBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A dropdown menu with file actions, separators, and a destructive delete item."
        </p>

        <DemoTabs source=extract_demo(include_str!("dropdown_menu_basic.rs"))>
            <DropdownMenuBasic />
        </DemoTabs>
    }
}
