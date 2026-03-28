use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::AspectRatioBasicSection;

#[component]
pub fn AspectRatioPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Aspect Ratio"
                description="Displays content within a desired ratio."
                features=&["Responsive", "Custom ratios"]
            />

            <DocSection title="Demo">
                <AspectRatioBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="AspectRatio" description="Enforces a width-to-height ratio on its child content. Defaults to 1:1." />
                </div>
            </DocSection>
        </div>
    }
}
