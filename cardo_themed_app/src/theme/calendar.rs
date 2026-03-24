use cardo_ui::calendar::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york calendar
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "bg-background p-3";

const HEADER_CLASS: &str = "flex items-center justify-between";

const HEADING_CLASS: &str = "text-sm font-medium";

const NAV_BUTTON_CLASS: &str = "inline-flex items-center justify-center size-7 rounded-md hover:bg-accent hover:text-accent-foreground disabled:disabled-base";

const GRID_CLASS: &str = "w-full border-collapse";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCalendar(
    #[prop(into, optional)] value: MaybeProp<NaiveDate>,
    #[prop(into, optional)] default_value: MaybeProp<NaiveDate>,
    #[prop(into, optional)] on_value_change: Option<Callback<NaiveDate>>,
    #[prop(into, optional)] month: MaybeProp<NaiveDate>,
    #[prop(into, optional)] default_month: MaybeProp<NaiveDate>,
    #[prop(into, optional)] on_month_change: Option<Callback<NaiveDate>>,
    #[prop(into, optional)] min_date: MaybeProp<NaiveDate>,
    #[prop(into, optional)] max_date: MaybeProp<NaiveDate>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ROOT_CLASS);

    view! {
        <Calendar
            attr:class=class.get_value()
            value=value
            default_value=default_value
            on_value_change=move |val: NaiveDate| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
            month=month
            default_month=default_month
            on_month_change=move |val: NaiveDate| {
                if let Some(cb) = on_month_change {
                    cb.run(val);
                }
            }
            min_date=min_date
            max_date=max_date
            disabled=disabled
        >
            {children()}
        </Calendar>
    }
}

#[component]
pub fn ThemedCalendarHeader(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(HEADER_CLASS);

    view! {
        <CalendarHeader attr:class=class.get_value()>
            {children()}
        </CalendarHeader>
    }
}

#[component]
pub fn ThemedCalendarHeading() -> impl IntoView {
    let class = StoredValue::new(HEADING_CLASS);

    view! {
        <CalendarHeading attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedCalendarPrevButton() -> impl IntoView {
    let class = StoredValue::new(NAV_BUTTON_CLASS);

    view! {
        <CalendarPrevButton attr:class=class.get_value()>
            <ChevronLeftIcon />
        </CalendarPrevButton>
    }
}

#[component]
pub fn ThemedCalendarNextButton() -> impl IntoView {
    let class = StoredValue::new(NAV_BUTTON_CLASS);

    view! {
        <CalendarNextButton attr:class=class.get_value()>
            <ChevronRightIcon />
        </CalendarNextButton>
    }
}

#[component]
pub fn ThemedCalendarGrid(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(GRID_CLASS);

    view! {
        <CalendarGrid attr:class=class.get_value()>
            {children()}
        </CalendarGrid>
    }
}

#[component]
pub fn ThemedCalendarGridHead() -> impl IntoView {
    view! {
        <CalendarGridHead />
    }
}

#[component]
pub fn ThemedCalendarGridBody() -> impl IntoView {
    view! {
        <CalendarGridBody />
    }
}

// ---------------------------------------------------------------------------
// Icons
// ---------------------------------------------------------------------------

#[component]
fn ChevronLeftIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m15 18-6-6 6-6" />
        </svg>
    }
}

#[component]
fn ChevronRightIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m9 18 6-6-6-6" />
        </svg>
    }
}
