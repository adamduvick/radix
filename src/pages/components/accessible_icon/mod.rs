use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::AccessibleIconBasicSection;

#[component]
pub fn AccessibleIconPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Accessible Icon"
                description="Makes an icon accessible by adding a label for screen readers and hiding the icon from the accessibility tree."
                features=&["Accessible", "Screen reader label", "Hides decorative SVG"]
            />

            <DocSection title="Demo">
                <AccessibleIconBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="AccessibleIcon" description="Wraps an icon, hides it from screen readers, and exposes a text label instead." />
                </div>
            </DocSection>
        </div>
    }
}
