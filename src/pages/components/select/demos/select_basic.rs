use leptos::prelude::*;
use pith_ui::select::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "inline-flex items-center justify-between gap-2 rounded-lg \
    border border-slate-300 bg-white px-3 py-2 text-sm text-slate-700 shadow-sm \
    hover:bg-slate-50 min-w-[180px]";
const CONTENT: &str = "select-content overflow-hidden rounded-lg border \
    border-slate-200 bg-white shadow-lg";
const ITEM: &str = "relative flex cursor-pointer select-none items-center rounded-md \
    px-2 py-1.5 pl-8 text-sm text-slate-700 outline-none \
    data-[highlighted]:bg-accent-50 data-[highlighted]:text-accent-700";
const LABEL: &str = "px-6 py-1.5 text-xs font-semibold text-slate-500";
const INDICATOR: &str = "absolute left-2 inline-flex items-center";

fn chevron_icon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
            stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round">
            <path d="m4 5 3 3 3-3" />
        </svg>
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
pub fn SelectBasic() -> impl IntoView {
    view! {
        <Select default_value="apple">
            <SelectTrigger attr:class=TRIGGER>
                <SelectValue />
                <SelectIcon attr:class="text-slate-400">
                    {chevron_icon()}
                </SelectIcon>
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class=CONTENT position="popper" side_offset=4.0>
                    <SelectViewport attr:class="p-1">
                        <SelectGroup>
                            <SelectLabel attr:class=LABEL>"Fruits"</SelectLabel>
                            <SelectItem value="apple" attr:class=ITEM>
                                <SelectItemIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </SelectItemIndicator>
                                <SelectItemText>"Apple"</SelectItemText>
                            </SelectItem>
                            <SelectItem value="banana" attr:class=ITEM>
                                <SelectItemIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </SelectItemIndicator>
                                <SelectItemText>"Banana"</SelectItemText>
                            </SelectItem>
                            <SelectItem value="orange" attr:class=ITEM>
                                <SelectItemIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </SelectItemIndicator>
                                <SelectItemText>"Orange"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>
                        <SelectSeparator attr:class="mx-1 my-1 h-px bg-slate-200" />
                        <SelectGroup>
                            <SelectLabel attr:class=LABEL>"Vegetables"</SelectLabel>
                            <SelectItem value="carrot" attr:class=ITEM>
                                <SelectItemIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </SelectItemIndicator>
                                <SelectItemText>"Carrot"</SelectItemText>
                            </SelectItem>
                            <SelectItem value="broccoli" attr:class=ITEM>
                                <SelectItemIndicator attr:class=INDICATOR>
                                    {check_icon()}
                                </SelectItemIndicator>
                                <SelectItemText>"Broccoli"</SelectItemText>
                            </SelectItem>
                        </SelectGroup>
                    </SelectViewport>
                </SelectContent>
            </SelectPortal>
        </Select>
    }
}
// #endregion demo

#[component]
pub fn SelectBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A dropdown select with grouped options, item indicators, and keyboard navigation."
        </p>

        <DemoTabs source=extract_demo(include_str!("select_basic.rs"))>
            <SelectBasic />
        </DemoTabs>
    }
}
