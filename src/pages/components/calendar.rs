use leptos::prelude::*;
use pith_ui::calendar::*;

use crate::components::*;

#[component]
pub fn CalendarPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Calendar"
                description="A date picker calendar for selecting a single date."
                features=&["Accessible", "Keyboard navigation", "Configurable week start", "Min/max dates", "Controlled & uncontrolled"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <Calendar attr:class="inline-block rounded-lg border border-slate-200 p-3">
                        <CalendarHeader attr:class="flex items-center justify-between mb-2">
                            <CalendarPrevButton attr:class="inline-flex h-8 w-8 items-center justify-center rounded-md text-slate-600 hover:bg-slate-100 transition-colors">
                                {left_arrow()}
                            </CalendarPrevButton>
                            <CalendarHeading attr:class="text-sm font-semibold text-slate-900" />
                            <CalendarNextButton attr:class="inline-flex h-8 w-8 items-center justify-center rounded-md text-slate-600 hover:bg-slate-100 transition-colors">
                                {right_arrow()}
                            </CalendarNextButton>
                        </CalendarHeader>
                        <CalendarGrid attr:class="w-full border-collapse [&_th]:w-9 [&_th]:text-xs [&_th]:font-medium [&_th]:text-slate-500 [&_th]:pb-1 [&_td]:p-0 [&_button]:h-9 [&_button]:w-9 [&_button]:rounded-md [&_button]:text-sm [&_button]:text-slate-700 [&_button:hover]:bg-accent-50 [&_button[data-selected]]:bg-accent-600 [&_button[data-selected]]:text-white [&_button[data-today]]:font-bold [&_button[data-outside-month]]:text-slate-300 [&_button:disabled]:text-slate-300 [&_button:disabled]:pointer-events-none">
                            <CalendarGridHead />
                            <CalendarGridBody />
                        </CalendarGrid>
                    </Calendar>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::calendar::*;

view! {
    <Calendar>
        <CalendarHeader>
            <CalendarPrevButton>"<"</CalendarPrevButton>
            <CalendarHeading />
            <CalendarNextButton>">"</CalendarNextButton>
        </CalendarHeader>
        <CalendarGrid>
            <CalendarGridHead />
            <CalendarGridBody />
        </CalendarGrid>
    </Calendar>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Calendar" description="Root component. Manages selected date and displayed month." />
                    <PartItem name="CalendarHeader" description="Container for navigation and heading." />
                    <PartItem name="CalendarPrevButton" description="Navigates to the previous month." />
                    <PartItem name="CalendarHeading" description="Displays the current month and year." />
                    <PartItem name="CalendarNextButton" description="Navigates to the next month." />
                    <PartItem name="CalendarGrid" description="The table element containing the date grid." />
                    <PartItem name="CalendarGridHead" description="Renders the weekday header row." />
                    <PartItem name="CalendarGridBody" description="Renders the date cells." />
                </div>
            </DocSection>
        </div>
    }
}

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
