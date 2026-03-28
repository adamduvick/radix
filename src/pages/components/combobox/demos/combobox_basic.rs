use leptos::prelude::*;
use pith_ui::combobox::*;

use crate::components::{extract_demo, DemoTabs};

const FRUITS: &[&str] = &[
    "Apple", "Avocado", "Banana", "Blueberry", "Cherry", "Grape", "Kiwi", "Lemon", "Mango",
    "Orange", "Peach", "Pear", "Strawberry",
];

const ANCHOR: &str = "flex items-center rounded-lg border border-slate-300 bg-white \
    pr-1 focus-within:border-accent-500 focus-within:ring-1 \
    focus-within:ring-accent-500";
const INPUT: &str = "flex-1 rounded-lg border-0 bg-transparent px-3 py-2 text-sm \
    text-slate-700 placeholder:text-slate-400 focus:outline-none";
const TRIGGER: &str = "inline-flex h-7 w-7 items-center justify-center rounded-md \
    text-slate-400 hover:text-slate-600";
const CONTENT: &str = "select-content mt-1 overflow-hidden rounded-lg border \
    border-slate-200 bg-white shadow-lg";
const VIEWPORT: &str = "max-h-60 p-1";
const EMPTY: &str = "px-3 py-2 text-center text-sm text-slate-500";
const ITEM: &str = "relative flex cursor-pointer select-none items-center rounded-md \
    px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none \
    data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700";
const ITEM_INDICATOR: &str = "absolute left-2 inline-flex items-center";

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

fn check_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <path d="m3 7 3 3 5-5" />
        </svg>
    }
}

// #region demo
#[component]
pub fn ComboboxBasic() -> impl IntoView {
    let (value, set_value) = signal(Option::<String>::None);
    let (input_value, set_input_value) = signal(String::new());
    let filtered = Memo::new(move |_| filter(FRUITS, &input_value.get()));

    view! {
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
                <ComboboxAnchor attr:class=ANCHOR>
                    <ComboboxInput
                        attr:class=INPUT
                        attr:placeholder="Search fruits..."
                    />
                    <ComboboxTrigger attr:class=TRIGGER>
                        <ComboboxIcon />
                    </ComboboxTrigger>
                </ComboboxAnchor>
                <ComboboxPortal>
                    <ComboboxContent
                        attr:class=CONTENT
                        side_offset=4.0
                    >
                        <ComboboxViewport attr:class=VIEWPORT>
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    return leptos::either::Either::Left(view! {
                                        <ComboboxEmpty attr:class=EMPTY>
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
                                                attr:class=ITEM
                                            >
                                                <ComboboxItemIndicator attr:class=ITEM_INDICATOR>
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
    }
}
// #endregion demo

#[component]
pub fn ComboboxBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A searchable combobox with typeahead filtering over a list of fruits."
        </p>

        <DemoTabs source=extract_demo(include_str!("combobox_basic.rs"))>
            <ComboboxBasic />
        </DemoTabs>
    }
}
