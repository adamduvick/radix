use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::ContextMenuBasicSection;

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
                <ContextMenuBasicSection />
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
