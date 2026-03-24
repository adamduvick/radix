use leptos::prelude::*;

use crate::theme::combobox::*;

const FRAMEWORKS: &[(&str, &str)] = &[
    ("react", "React"),
    ("vue", "Vue"),
    ("angular", "Angular"),
    ("svelte", "Svelte"),
    ("leptos", "Leptos"),
];

#[component]
pub fn ComboboxPage() -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let (value, set_value) = signal::<Option<String>>(None);

    let filtered = Memo::new(move |_| {
        let query = search.get().to_lowercase();
        FRAMEWORKS
            .iter()
            .filter(|(_, label)| query.is_empty() || label.to_lowercase().contains(&query))
            .copied()
            .collect::<Vec<_>>()
    });

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Combobox"</h1>
                <p class="text-muted-foreground mb-6">
                    "An autocomplete input with a filterable dropdown list of options."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Framework Picker"</h2>
                <p class="text-sm text-muted-foreground">
                    "Selected: "
                    {move || value.get().unwrap_or_else(|| "None".to_string())}
                </p>
                <div class="w-[240px]">
                    <ThemedCombobox
                        value=MaybeProp::derive(move || value.get())
                        on_value_change=Callback::new(move |val: String| {
                            set_value.set(Some(val));
                        })
                        input_value=MaybeProp::derive(move || Some(search.get()))
                        on_input_value_change=Callback::new(move |val: String| {
                            set_search.set(val);
                        })
                    >
                        <ThemedComboboxAnchor>
                            <ThemedComboboxInput placeholder="Search frameworks..." />
                            <ThemedComboboxTrigger />
                        </ThemedComboboxAnchor>
                        <ThemedComboboxContent>
                            <ThemedComboboxViewport>
                                {move || {
                                    let items = filtered.get();
                                    if items.is_empty() {
                                        view! {
                                            <ThemedComboboxEmpty>"No framework found."</ThemedComboboxEmpty>
                                        }.into_any()
                                    } else {
                                        items
                                            .into_iter()
                                            .map(|(val, label)| {
                                                view! {
                                                    <ThemedComboboxItem value=val.to_string()>
                                                        {label}
                                                    </ThemedComboboxItem>
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }
                                }}
                            </ThemedComboboxViewport>
                        </ThemedComboboxContent>
                    </ThemedCombobox>
                </div>
            </section>
        </div>
    }
}
