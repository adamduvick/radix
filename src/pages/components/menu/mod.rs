use leptos::prelude::*;

use crate::components::*;

#[component]
pub fn MenuPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Menu"
                description="A low-level menu primitive used as the foundation for Dropdown Menu, Context Menu, and Menubar."
                features=&["Accessible", "Keyboard navigation", "Submenus", "Checkbox & radio items", "Typeahead"]
            />

            <DocSection title="Overview">
                <div class="rounded-lg border border-amber-200 bg-amber-50 p-4 text-sm text-amber-800">
                    <p class="font-semibold">"Building block primitive"</p>
                    <p class="mt-1">
                        "Menu is the base component used internally by "
                        <code class="rounded bg-amber-100 px-1 text-xs">"DropdownMenu"</code>", "
                        <code class="rounded bg-amber-100 px-1 text-xs">"ContextMenu"</code>", and "
                        <code class="rounded bg-amber-100 px-1 text-xs">"Menubar"</code>
                        ". For most use cases, prefer those higher-level components. Use Menu directly only when building custom menu-like patterns."
                    </p>
                </div>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::menu::*;

view! {
    <Menu>
        <MenuAnchor>
            <button>"Open"</button>
        </MenuAnchor>
        <MenuPortal>
            <MenuContent>
                <MenuItem>"Item 1"</MenuItem>
                <MenuItem>"Item 2"</MenuItem>
                <MenuSeparator />
                <MenuCheckboxItem>"Toggle option"</MenuCheckboxItem>
                <MenuSub>
                    <MenuSubTrigger>"More..."</MenuSubTrigger>
                    <MenuSubContent>
                        <MenuItem>"Sub-item"</MenuItem>
                    </MenuSubContent>
                </MenuSub>
            </MenuContent>
        </MenuPortal>
    </Menu>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Menu" description="Root component. Manages open state." />
                    <PartItem name="MenuAnchor" description="Positions the menu relative to this element." />
                    <PartItem name="MenuPortal" description="Portals content to the body." />
                    <PartItem name="MenuContent" description="The floating menu panel." />
                    <PartItem name="MenuArrow" description="An arrow pointing to the anchor." />
                    <PartItem name="MenuItem" description="An actionable menu item." />
                    <PartItem name="MenuCheckboxItem" description="A toggleable checkbox item." />
                    <PartItem name="MenuRadioGroup" description="Groups radio items together." />
                    <PartItem name="MenuRadioItem" description="A radio-style selectable item." />
                    <PartItem name="MenuItemIndicator" description="Visual indicator for checked items." />
                    <PartItem name="MenuGroup" description="Groups related items." />
                    <PartItem name="MenuLabel" description="A non-interactive label." />
                    <PartItem name="MenuSeparator" description="A visual divider." />
                    <PartItem name="MenuSub" description="Container for a submenu." />
                    <PartItem name="MenuSubTrigger" description="Item that opens a submenu." />
                    <PartItem name="MenuSubContent" description="The submenu content panel." />
                </div>
            </DocSection>
        </div>
    }
}
