use leptos::prelude::*;
use pith_ui::dropdown_menu::*;

use crate::components::*;

#[component]
pub fn DropdownMenuPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Dropdown Menu"
                description="A menu of actions triggered by a button, supporting submenus, checkbox items, and radio items."
                features=&["Accessible", "Keyboard navigation", "Submenus", "Checkbox & radio items", "Typeahead"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <DropdownMenu>
                        <DropdownMenuTrigger attr:class="inline-flex items-center gap-1 rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 shadow-sm hover:bg-slate-50 transition-colors">
                            "Options"
                            {chevron_icon()}
                        </DropdownMenuTrigger>
                        <DropdownMenuPortal>
                            <DropdownMenuContent
                                attr:class="dropdown-content min-w-[180px] rounded-lg border border-slate-200 bg-white p-1 shadow-lg"
                                side_offset=4.0
                            >
                                <DropdownMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "New File"
                                </DropdownMenuItem>
                                <DropdownMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Open..."
                                </DropdownMenuItem>
                                <DropdownMenuSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                <DropdownMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Save"
                                </DropdownMenuItem>
                                <DropdownMenuItem attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                    "Save As..."
                                </DropdownMenuItem>
                                <DropdownMenuSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                <DropdownMenuItem
                                    attr:class="flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 text-sm text-red-600 outline-none data-[highlighted]:bg-red-50"
                                >
                                    "Delete"
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenuPortal>
                    </DropdownMenu>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::dropdown_menu::*;

view! {
    <DropdownMenu>
        <DropdownMenuTrigger>"Options"</DropdownMenuTrigger>
        <DropdownMenuPortal>
            <DropdownMenuContent side_offset=4.0>
                <DropdownMenuItem>"New File"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Delete"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenuPortal>
    </DropdownMenu>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="DropdownMenu" description="Root component. Manages open state." />
                    <PartItem name="DropdownMenuTrigger" description="Button that opens the menu." />
                    <PartItem name="DropdownMenuPortal" description="Portals content to the body." />
                    <PartItem name="DropdownMenuContent" description="The floating menu panel." />
                    <PartItem name="DropdownMenuItem" description="An actionable menu item." />
                    <PartItem name="DropdownMenuCheckboxItem" description="A toggleable checkbox item." />
                    <PartItem name="DropdownMenuRadioGroup" description="Groups radio items together." />
                    <PartItem name="DropdownMenuRadioItem" description="A radio-style selectable item." />
                    <PartItem name="DropdownMenuSeparator" description="A visual divider between items." />
                    <PartItem name="DropdownMenuLabel" description="A non-interactive label for grouping." />
                    <PartItem name="DropdownMenuSub" description="Container for a submenu." />
                    <PartItem name="DropdownMenuSubTrigger" description="Item that opens a submenu." />
                    <PartItem name="DropdownMenuSubContent" description="The submenu content panel." />
                </div>
            </DocSection>
        </div>
    }
}

fn chevron_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-slate-400">
            <path d="m4 5 3 3 3-3" />
        </svg>
    }
}
