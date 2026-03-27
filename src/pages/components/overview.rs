use leptos::prelude::*;
use leptos_router::components::A;
use pith_ui::separator::Separator;

#[component]
pub fn ComponentsOverview() -> impl IntoView {
    view! {
        <div class="max-w-4xl">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight text-slate-900">"Components"</h1>
                <p class="mt-2 text-lg text-slate-600">
                    "Unstyled, accessible UI primitives for Leptos applications."
                </p>
                <Separator attr:class="mt-6 h-px bg-slate-200" decorative=true />
            </div>

            <ComponentGroup
                title="Layout"
                items=vec![
                    ("Separator", "/docs/components/separator", "Visually or semantically separates content."),
                ]
            />

            <ComponentGroup
                title="Inputs"
                items=vec![
                    ("Checkbox", "/docs/components/checkbox", "A control that toggles between checked and unchecked."),
                    ("Label", "/docs/components/label", "Accessible label for form controls."),
                    ("Radio Group", "/docs/components/radio-group", "A set of mutually exclusive options."),
                    ("Select", "/docs/components/select", "A dropdown for selecting from a list of options."),
                    ("Slider", "/docs/components/slider", "A range input for selecting numeric values."),
                    ("Switch", "/docs/components/switch", "A toggle between on and off states."),
                    ("Toggle", "/docs/components/toggle", "A two-state button that can be on or off."),
                    ("Toggle Group", "/docs/components/toggle-group", "A group of toggle buttons."),
                ]
            />

            <ComponentGroup
                title="Overlays"
                items=vec![
                    ("Alert Dialog", "/docs/components/alert-dialog", "A modal dialog that interrupts the user for confirmation."),
                    ("Dialog", "/docs/components/dialog", "A modal or non-modal overlay window."),
                    ("Dropdown Menu", "/docs/components/dropdown-menu", "A menu triggered by a button."),
                    ("Popover", "/docs/components/popover", "Floating content anchored to a trigger."),
                    ("Toast", "/docs/components/toast", "A brief, auto-dismissing notification."),
                    ("Tooltip", "/docs/components/tooltip", "A popup label on hover or focus."),
                ]
            />

            <ComponentGroup
                title="Disclosure"
                items=vec![
                    ("Accordion", "/docs/components/accordion", "Vertically stacked expandable sections."),
                    ("Collapsible", "/docs/components/collapsible", "A panel that expands and collapses."),
                    ("Tabs", "/docs/components/tabs", "Tabbed interface for switching between views."),
                ]
            />

            <ComponentGroup
                title="Data Display"
                items=vec![
                    ("Avatar", "/docs/components/avatar", "An image element with a fallback."),
                    ("Progress", "/docs/components/progress", "Displays progress toward completion."),
                ]
            />
        </div>
    }
}

#[component]
fn ComponentGroup(
    title: &'static str,
    items: Vec<(&'static str, &'static str, &'static str)>,
) -> impl IntoView {
    view! {
        <section class="mb-10">
            <h2 class="mb-4 text-lg font-semibold text-slate-900">{title}</h2>
            <div class="grid gap-3 sm:grid-cols-2">
                {items.into_iter().map(|(name, href, desc)| view! {
                    <A
                        href=href
                        attr:class="group rounded-lg border border-slate-200 p-4 no-underline hover:border-accent-300 hover:bg-accent-50/50 transition-all"
                    >
                        <h3 class="text-sm font-semibold text-slate-900 group-hover:text-accent-700">{name}</h3>
                        <p class="mt-1 text-xs text-slate-500">{desc}</p>
                    </A>
                }).collect_view()}
            </div>
        </section>
    }
}
