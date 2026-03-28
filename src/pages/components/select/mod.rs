use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::SelectBasicSection;

#[component]
pub fn SelectPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Select"
                description="A control that allows the user to choose one option from a dropdown list."
                features=&["Accessible", "Keyboard navigation", "Typeahead", "Groups", "Scroll buttons"]
            />

            <DocSection title="Demo">
                <SelectBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Select" description="Root component. Manages the selected value." />
                    <PartItem name="SelectTrigger" description="The button that opens the dropdown." />
                    <PartItem name="SelectValue" description="Displays the selected value text." />
                    <PartItem name="SelectIcon" description="A small icon next to the value (usually a chevron)." />
                    <PartItem name="SelectPortal" description="Portals dropdown content to the body." />
                    <PartItem name="SelectContent" description="The dropdown panel containing items." />
                    <PartItem name="SelectViewport" description="Scrollable area within the dropdown." />
                    <PartItem name="SelectGroup" description="Groups related items together." />
                    <PartItem name="SelectLabel" description="Label for a group of items." />
                    <PartItem name="SelectItem" description="An individual selectable option." />
                    <PartItem name="SelectItemText" description="The text content of an item." />
                    <PartItem name="SelectItemIndicator" description="Renders when the item is selected." />
                    <PartItem name="SelectSeparator" description="A visual separator between groups." />
                </div>
            </DocSection>
        </div>
    }
}
