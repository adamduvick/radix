use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::CalendarBasicSection;

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
                <CalendarBasicSection />
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
