use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::ComboboxBasicSection;

#[component]
pub fn ComboboxPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Combobox"
                description="An input with an associated popup listbox for selecting values, with typeahead filtering."
                features=&["Accessible", "Typeahead filtering", "Single & multi-select", "Keyboard navigation", "Virtual scrolling", "Groups"]
            />

            <DocSection title="Demo">
                <ComboboxBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Combobox" description="Root component. Manages value, input, and open state." />
                    <PartItem name="ComboboxAnchor" description="Wrapper around the input and trigger for positioning." />
                    <PartItem name="ComboboxInput" description="Text input with typeahead filtering." />
                    <PartItem name="ComboboxTrigger" description="Button that opens the dropdown." />
                    <PartItem name="ComboboxIcon" description="Chevron icon inside the trigger." />
                    <PartItem name="ComboboxClear" description="Button to clear the current value." />
                    <PartItem name="ComboboxChips" description="Container for multi-select chips." />
                    <PartItem name="ComboboxChip" description="A chip representing a selected value." />
                    <PartItem name="ComboboxChipRemove" description="Button to remove a chip." />
                    <PartItem name="ComboboxPortal" description="Portals dropdown content to the body." />
                    <PartItem name="ComboboxContent" description="The floating dropdown panel." />
                    <PartItem name="ComboboxViewport" description="Scrollable area within the dropdown." />
                    <PartItem name="ComboboxEmpty" description="Shown when no items match the filter." />
                    <PartItem name="ComboboxGroup" description="Groups related items." />
                    <PartItem name="ComboboxLabel" description="Label for a group." />
                    <PartItem name="ComboboxItem" description="An individual selectable option." />
                    <PartItem name="ComboboxItemText" description="The text content of an item." />
                    <PartItem name="ComboboxItemIndicator" description="Renders when the item is selected." />
                    <PartItem name="ComboboxSeparator" description="Visual separator between groups." />
                </div>
            </DocSection>
        </div>
    }
}
