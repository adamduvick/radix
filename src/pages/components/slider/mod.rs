use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::SliderBasicSection;

#[component]
pub fn SliderPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Slider"
                description="An input for selecting a numeric value from a range by dragging a thumb."
                features=&["Accessible", "Keyboard control", "Multiple thumbs", "Step support", "Vertical orientation"]
            />

            <DocSection title="Demo">
                <SliderBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Slider" description="Root component. Manages the value state." />
                    <PartItem name="SliderTrack" description="The track the thumb slides along." />
                    <PartItem name="SliderRange" description="The filled portion of the track." />
                    <PartItem name="SliderThumb" description="The draggable thumb. Add multiple for range sliders." />
                </div>
            </DocSection>
        </div>
    }
}
