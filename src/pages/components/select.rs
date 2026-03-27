use leptos::prelude::*;
use pith_ui::select::*;

use crate::components::*;

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
                <DemoSection>
                    <Select default_value="apple">
                        <SelectTrigger attr:class="inline-flex items-center justify-between gap-2 rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-700 shadow-sm hover:bg-slate-50 min-w-[180px]">
                            <SelectValue />
                            <SelectIcon attr:class="text-slate-400">
                                {chevron_icon()}
                            </SelectIcon>
                        </SelectTrigger>
                        <SelectPortal>
                            <SelectContent
                                attr:class="select-content overflow-hidden rounded-lg border border-slate-200 bg-white shadow-lg"
                                position="popper"
                                side_offset=4.0
                            >
                                <SelectViewport attr:class="p-1">
                                    <SelectGroup>
                                        <SelectLabel attr:class="px-6 py-1.5 text-xs font-semibold text-slate-500">"Fruits"</SelectLabel>
                                        <SelectItem value="apple" attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                            <SelectItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                {check_icon()}
                                            </SelectItemIndicator>
                                            <SelectItemText>"Apple"</SelectItemText>
                                        </SelectItem>
                                        <SelectItem value="banana" attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                            <SelectItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                {check_icon()}
                                            </SelectItemIndicator>
                                            <SelectItemText>"Banana"</SelectItemText>
                                        </SelectItem>
                                        <SelectItem value="orange" attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                            <SelectItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                {check_icon()}
                                            </SelectItemIndicator>
                                            <SelectItemText>"Orange"</SelectItemText>
                                        </SelectItem>
                                    </SelectGroup>
                                    <SelectSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                                    <SelectGroup>
                                        <SelectLabel attr:class="px-6 py-1.5 text-xs font-semibold text-slate-500">"Vegetables"</SelectLabel>
                                        <SelectItem value="carrot" attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                            <SelectItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                {check_icon()}
                                            </SelectItemIndicator>
                                            <SelectItemText>"Carrot"</SelectItemText>
                                        </SelectItem>
                                        <SelectItem value="broccoli" attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700">
                                            <SelectItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                {check_icon()}
                                            </SelectItemIndicator>
                                            <SelectItemText>"Broccoli"</SelectItemText>
                                        </SelectItem>
                                    </SelectGroup>
                                </SelectViewport>
                            </SelectContent>
                        </SelectPortal>
                    </Select>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::select::*;

view! {
    <Select default_value="apple">
        <SelectTrigger>
            <SelectValue />
            <SelectIcon />
        </SelectTrigger>
        <SelectPortal>
            <SelectContent position="popper">
                <SelectViewport>
                    <SelectItem value="apple">
                        <SelectItemText>"Apple"</SelectItemText>
                    </SelectItem>
                    <SelectItem value="banana">
                        <SelectItemText>"Banana"</SelectItemText>
                    </SelectItem>
                </SelectViewport>
            </SelectContent>
        </SelectPortal>
    </Select>
}"# />
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

fn chevron_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m4 5 3 3 3-3" />
        </svg>
    }
}

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}
