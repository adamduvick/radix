use leptos::prelude::*;
use pith_ui::menubar::*;

use crate::components::{extract_demo, DemoTabs};

const MENUBAR: &str = "flex rounded-lg border border-slate-200 bg-white p-0.5 shadow-sm";
const TRIGGER: &str = "rounded-md px-3 py-1.5 text-sm font-medium text-slate-700 \
    hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors";
const CONTENT: &str = "dropdown-content min-w-[180px] rounded-lg border border-slate-200 \
    bg-white p-1 shadow-lg";
const ITEM: &str = "flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 \
    text-sm text-slate-700 outline-none \
    data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700";
const SEPARATOR: &str = "mx-1 my-1 h-px bg-slate-200";

// #region demo
#[component]
pub fn MenubarBasic() -> impl IntoView {
    view! {
        <Menubar attr:class=MENUBAR>
            <MenubarMenu>
                <MenubarTrigger attr:class=TRIGGER>
                    "File"
                </MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent attr:class=CONTENT side_offset=4.0>
                        <MenubarItem attr:class=ITEM>"New File"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Open..."</MenubarItem>
                        <MenubarSeparator attr:class=SEPARATOR />
                        <MenubarItem attr:class=ITEM>"Save"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Exit"</MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>

            <MenubarMenu>
                <MenubarTrigger attr:class=TRIGGER>
                    "Edit"
                </MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent attr:class=CONTENT side_offset=4.0>
                        <MenubarItem attr:class=ITEM>"Undo"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Redo"</MenubarItem>
                        <MenubarSeparator attr:class=SEPARATOR />
                        <MenubarItem attr:class=ITEM>"Cut"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Copy"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Paste"</MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>

            <MenubarMenu>
                <MenubarTrigger attr:class=TRIGGER>
                    "View"
                </MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent attr:class=CONTENT side_offset=4.0>
                        <MenubarItem attr:class=ITEM>"Zoom In"</MenubarItem>
                        <MenubarItem attr:class=ITEM>"Zoom Out"</MenubarItem>
                        <MenubarSeparator attr:class=SEPARATOR />
                        <MenubarItem attr:class=ITEM>"Fullscreen"</MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>
        </Menubar>
    }
}
// #endregion demo

#[component]
pub fn MenubarBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A horizontal menu bar with File, Edit, and View dropdown menus."
        </p>

        <DemoTabs source=extract_demo(include_str!("menubar_basic.rs"))>
            <MenubarBasic />
        </DemoTabs>
    }
}
