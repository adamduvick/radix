use leptos::prelude::*;
use pith_ui::menubar::*;

use crate::components::*;

#[component]
pub fn MenubarPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Menubar"
                description="A horizontal menu bar with dropdown menus, commonly used for application-level navigation."
                features=&["Accessible", "Keyboard navigation", "Submenus", "Checkbox & radio items", "Arrow key traversal"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <Menubar attr:class="flex rounded-lg border border-slate-200 bg-white p-0.5 shadow-sm">
                        <MenubarMenu>
                            <MenubarTrigger attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                                "File"
                            </MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent attr:class="dropdown-content min-w-[180px] rounded-lg border border-slate-200 bg-white p-1 shadow-lg" side_offset=4.0>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"New File"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Open..."</MenubarItem>
                                    <MenubarSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Save"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Exit"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>

                        <MenubarMenu>
                            <MenubarTrigger attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                                "Edit"
                            </MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent attr:class="dropdown-content min-w-[180px] rounded-lg border border-slate-200 bg-white p-1 shadow-lg" side_offset=4.0>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Undo"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Redo"</MenubarItem>
                                    <MenubarSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Cut"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Copy"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Paste"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>

                        <MenubarMenu>
                            <MenubarTrigger attr:class="rounded-md px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                                "View"
                            </MenubarTrigger>
                            <MenubarPortal>
                                <MenubarContent attr:class="dropdown-content min-w-[180px] rounded-lg border border-slate-200 bg-white p-1 shadow-lg" side_offset=4.0>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Zoom In"</MenubarItem>
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Zoom Out"</MenubarItem>
                                    <MenubarSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                    <MenubarItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">"Fullscreen"</MenubarItem>
                                </MenubarContent>
                            </MenubarPortal>
                        </MenubarMenu>
                    </Menubar>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::menubar::*;

view! {
    <Menubar>
        <MenubarMenu>
            <MenubarTrigger>"File"</MenubarTrigger>
            <MenubarPortal>
                <MenubarContent>
                    <MenubarItem>"New"</MenubarItem>
                    <MenubarItem>"Open"</MenubarItem>
                </MenubarContent>
            </MenubarPortal>
        </MenubarMenu>
    </Menubar>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Menubar" description="Root container for the menu bar." />
                    <PartItem name="MenubarMenu" description="A top-level menu within the bar." />
                    <PartItem name="MenubarTrigger" description="Button that opens the dropdown." />
                    <PartItem name="MenubarPortal" description="Portals content to the body." />
                    <PartItem name="MenubarContent" description="The dropdown panel." />
                    <PartItem name="MenubarItem" description="An actionable menu item." />
                    <PartItem name="MenubarCheckboxItem" description="A toggleable checkbox item." />
                    <PartItem name="MenubarRadioGroup" description="Groups radio items." />
                    <PartItem name="MenubarRadioItem" description="A radio-style item." />
                    <PartItem name="MenubarItemIndicator" description="Visual indicator for checked items." />
                    <PartItem name="MenubarSeparator" description="Visual divider between items." />
                    <PartItem name="MenubarLabel" description="Non-interactive label." />
                    <PartItem name="MenubarSub" description="Container for a submenu." />
                    <PartItem name="MenubarSubTrigger" description="Item that opens a submenu." />
                    <PartItem name="MenubarSubContent" description="Submenu content panel." />
                </div>
            </DocSection>
        </div>
    }
}
