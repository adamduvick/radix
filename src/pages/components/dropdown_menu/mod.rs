use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::DropdownMenuBasicSection;

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
                <DropdownMenuBasicSection />
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
