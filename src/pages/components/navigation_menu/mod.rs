use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::NavigationMenuBasicSection;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Navigation Menu"
                description="A collection of links for navigating websites, with support for expandable content panels."
                features=&["Accessible", "Keyboard navigation", "Submenus with viewport", "Active indicator", "RTL support"]
            />

            <DocSection title="Demo">
                <NavigationMenuBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="NavigationMenu" description="Root component. Manages open state." />
                    <PartItem name="NavigationMenuList" description="Container for menu items." />
                    <PartItem name="NavigationMenuItem" description="A top-level item (expandable or link)." />
                    <PartItem name="NavigationMenuTrigger" description="Button that opens content panel." />
                    <PartItem name="NavigationMenuContent" description="The expandable content panel." />
                    <PartItem name="NavigationMenuLink" description="A navigation link." />
                    <PartItem name="NavigationMenuSub" description="A sub-navigation for secondary lists." />
                    <PartItem name="NavigationMenuIndicator" description="Active indicator under the trigger." />
                    <PartItem name="NavigationMenuViewport" description="Shared viewport for content panels." />
                </div>
            </DocSection>
        </div>
    }
}
