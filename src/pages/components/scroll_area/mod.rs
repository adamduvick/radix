use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::ScrollAreaBasicSection;

#[component]
pub fn ScrollAreaPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Scroll Area"
                description="A custom scrollable area with styled scrollbars that augments native scroll functionality."
                features=&["Accessible", "Custom scrollbar styling", "Horizontal & vertical", "Auto-hide scrollbars", "Corner element"]
            />

            <DocSection title="Demo">
                <ScrollAreaBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="ScrollArea" description="Root component. Sets scrollbar type (auto, always, scroll, hover)." />
                    <PartItem name="ScrollAreaViewport" description="The scrollable content area." />
                    <PartItem name="ScrollAreaScrollbar" description="A custom scrollbar track (vertical or horizontal)." />
                    <PartItem name="ScrollAreaThumb" description="The draggable scrollbar thumb." />
                    <PartItem name="ScrollAreaCorner" description="Corner element when both scrollbars are visible." />
                </div>
            </DocSection>
        </div>
    }
}
