use leptos::prelude::*;
use pith_ui::calendar::*;

use crate::components::{extract_demo, DemoTabs};

const ROOT: &str = "inline-block rounded-lg border border-slate-200 p-3";
const HEADER: &str = "flex items-center justify-between mb-2";
const NAV_BUTTON: &str = "inline-flex h-8 w-8 items-center justify-center rounded-md \
    text-slate-600 hover:bg-slate-100 transition-colors";
const HEADING: &str = "text-sm font-semibold text-slate-900";
const GRID: &str = "w-full border-collapse \
    [&_th]:w-9 [&_th]:text-xs [&_th]:font-medium [&_th]:text-slate-500 [&_th]:pb-1 \
    [&_td]:p-0 \
    [&_button]:h-9 [&_button]:w-9 [&_button]:rounded-md [&_button]:text-sm [&_button]:text-slate-700 \
    [&_button:hover]:bg-accent-50 \
    [&_button[data-selected]]:bg-accent-600 [&_button[data-selected]]:text-white \
    [&_button[data-today]]:font-bold \
    [&_button[data-outside-month]]:text-slate-300 \
    [&_button:disabled]:text-slate-300 [&_button:disabled]:pointer-events-none";

fn left_arrow() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m9 3-4 4 4 4" />
        </svg>
    }
}

fn right_arrow() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="m5 3 4 4-4 4" />
        </svg>
    }
}

// #region demo
#[component]
pub fn CalendarBasic() -> impl IntoView {
    view! {
        <Calendar attr:class=ROOT>
            <CalendarHeader attr:class=HEADER>
                <CalendarPrevButton attr:class=NAV_BUTTON>
                    {left_arrow()}
                </CalendarPrevButton>
                <CalendarHeading attr:class=HEADING />
                <CalendarNextButton attr:class=NAV_BUTTON>
                    {right_arrow()}
                </CalendarNextButton>
            </CalendarHeader>
            <CalendarGrid attr:class=GRID>
                <CalendarGridHead />
                <CalendarGridBody />
            </CalendarGrid>
        </Calendar>
    }
}
// #endregion demo

#[component]
pub fn CalendarBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A single-date calendar with month navigation and keyboard support."
        </p>

        <DemoTabs source=extract_demo(include_str!("calendar_basic.rs"))>
            <CalendarBasic />
        </DemoTabs>
    }
}
