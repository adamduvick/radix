use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::MenubarBasicSection;

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
                <MenubarBasicSection />
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
