use leptos::prelude::*;
use pith_ui::context_menu::*;

use crate::components::*;

#[component]
pub fn ContextMenuPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Context Menu"
                description="A menu that appears on right-click, supporting submenus, checkbox items, and radio items."
                features=&["Accessible", "Right-click trigger", "Submenus", "Checkbox & radio items", "Keyboard navigation"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <ContextMenu>
                        <ContextMenuTrigger attr:class="flex h-36 w-full items-center justify-center rounded-lg border-2 border-dashed border-slate-300 text-sm text-slate-500 select-none">
                            "Right-click here"
                        </ContextMenuTrigger>
                        <ContextMenuPortal>
                            <ContextMenuContent attr:class="dropdown-content min-w-[180px] rounded-lg border border-slate-200 bg-white p-1 shadow-lg">
                                <ContextMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Back"
                                </ContextMenuItem>
                                <ContextMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Forward"
                                </ContextMenuItem>
                                <ContextMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Reload"
                                </ContextMenuItem>
                                <ContextMenuSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                <ContextMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "View Source"
                                </ContextMenuItem>
                                <ContextMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Inspect"
                                </ContextMenuItem>
                            </ContextMenuContent>
                        </ContextMenuPortal>
                    </ContextMenu>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::context_menu::*;

view! {
    <ContextMenu>
        <ContextMenuTrigger>"Right-click here"</ContextMenuTrigger>
        <ContextMenuPortal>
            <ContextMenuContent>
                <ContextMenuItem>"Back"</ContextMenuItem>
                <ContextMenuItem>"Forward"</ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem>"Inspect"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenuPortal>
    </ContextMenu>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="ContextMenu" description="Root component. Manages open state." />
                    <PartItem name="ContextMenuTrigger" description="The area that responds to right-click." />
                    <PartItem name="ContextMenuPortal" description="Portals content to the body." />
                    <PartItem name="ContextMenuContent" description="The floating menu panel." />
                    <PartItem name="ContextMenuItem" description="An actionable menu item." />
                    <PartItem name="ContextMenuCheckboxItem" description="A toggleable checkbox item." />
                    <PartItem name="ContextMenuRadioGroup" description="Groups radio items together." />
                    <PartItem name="ContextMenuRadioItem" description="A radio-style selectable item." />
                    <PartItem name="ContextMenuItemIndicator" description="Visual indicator for checked items." />
                    <PartItem name="ContextMenuSeparator" description="A visual divider between items." />
                    <PartItem name="ContextMenuLabel" description="A non-interactive label for grouping." />
                    <PartItem name="ContextMenuSub" description="Container for a submenu." />
                    <PartItem name="ContextMenuSubTrigger" description="Item that opens a submenu." />
                    <PartItem name="ContextMenuSubContent" description="The submenu content panel." />
                </div>
            </DocSection>
        </div>
    }
}
