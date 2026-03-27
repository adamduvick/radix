use leptos::prelude::*;
use pith_ui::combobox::*;

use crate::components::*;

const FRUITS: &[&str] = &[
    "Apple", "Avocado", "Banana", "Blueberry", "Cherry", "Grape", "Kiwi", "Lemon", "Mango",
    "Orange", "Peach", "Pear", "Strawberry",
];

fn filter(items: &[&str], query: &str) -> Vec<String> {
    if query.is_empty() {
        items.iter().map(|s| s.to_string()).collect()
    } else {
        let q = query.to_lowercase();
        items
            .iter()
            .filter(|i| i.to_lowercase().contains(&q))
            .map(|s| s.to_string())
            .collect()
    }
}

#[component]
pub fn ComboboxPage() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());
    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Combobox"
                description="An input with an associated popup listbox for selecting values, with typeahead filtering."
                features=&["Accessible", "Typeahead filtering", "Single & multi-select", "Keyboard navigation", "Virtual scrolling", "Groups"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="max-w-xs">
                        <label class="mb-1.5 block text-sm font-medium text-slate-700">"Favorite fruit"</label>
                        <Combobox
                            value=Signal::derive(move || value.get())
                            on_value_change=Callback::new(move |v: String| {
                                set_value.set(Some(v.clone()));
                                set_input_value.set(v);
                            })
                            input_value=Signal::derive(move || input_value.get())
                            on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
                        >
                            <ComboboxAnchor attr:class="flex items-center rounded-lg border border-slate-300 bg-white pr-1 focus-within:border-accent-500 focus-within:ring-1 focus-within:ring-accent-500">
                                <ComboboxInput
                                    attr:class="flex-1 rounded-lg border-0 bg-transparent px-3 py-2 text-sm text-slate-700 placeholder:text-slate-400 focus:outline-none"
                                    attr:placeholder="Search fruits..."
                                />
                                <ComboboxTrigger attr:class="inline-flex h-7 w-7 items-center justify-center rounded-md text-slate-400 hover:text-slate-600">
                                    <ComboboxIcon />
                                </ComboboxTrigger>
                            </ComboboxAnchor>
                            <ComboboxPortal>
                                <ComboboxContent
                                    attr:class="select-content mt-1 overflow-hidden rounded-lg border border-slate-200 bg-white shadow-lg"
                                    side_offset=4.0
                                >
                                    <ComboboxViewport attr:class="max-h-60 p-1">
                                        {move || {
                                            let items = filtered.get();
                                            if items.is_empty() {
                                                return leptos::either::Either::Left(view! {
                                                    <ComboboxEmpty attr:class="px-3 py-2 text-center text-sm text-slate-500">
                                                        "No fruits found."
                                                    </ComboboxEmpty>
                                                });
                                            }
                                            leptos::either::Either::Right(
                                                items.into_iter().map(|item| {
                                                    let text = item.clone();
                                                    let label = StoredValue::new(item.clone());
                                                    view! {
                                                        <ComboboxItem
                                                            value=item
                                                            text_value=text
                                                            attr:class="relative flex cursor-pointer select-none items-center rounded-md px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700"
                                                        >
                                                            <ComboboxItemIndicator attr:class="absolute left-2 inline-flex items-center">
                                                                {check_icon()}
                                                            </ComboboxItemIndicator>
                                                            {move || label.get_value()}
                                                        </ComboboxItem>
                                                    }
                                                }).collect_view()
                                            )
                                        }}
                                    </ComboboxViewport>
                                </ComboboxContent>
                            </ComboboxPortal>
                        </Combobox>
                        <p class="mt-2 text-xs text-slate-500">
                            "Selected: " {move || value.get().unwrap_or_else(|| "(none)".into())}
                        </p>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::combobox::*;

view! {
    <Combobox
        value=Signal::derive(move || value.get())
        on_value_change=Callback::new(move |v: String| set_value.set(Some(v)))
        input_value=Signal::derive(move || input_value.get())
        on_input_value_change=Callback::new(move |v: String| set_input_value.set(v))
    >
        <ComboboxAnchor>
            <ComboboxInput attr:placeholder="Search..." />
            <ComboboxTrigger><ComboboxIcon /></ComboboxTrigger>
        </ComboboxAnchor>
        <ComboboxPortal>
            <ComboboxContent>
                <ComboboxViewport>
                    <ComboboxItem value="apple" text_value="Apple">
                        <ComboboxItemIndicator />
                        "Apple"
                    </ComboboxItem>
                </ComboboxViewport>
            </ComboboxContent>
        </ComboboxPortal>
    </Combobox>
}"# />
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

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}
