use leptos::prelude::*;
use pith_ui::slider::{Slider, SliderRange, SliderThumb, SliderTrack};

use crate::components::*;

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
                <DemoSection>
                    <div class="space-y-8 max-w-md">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Volume"</label>
                            <Slider
                                default_value=vec![50.0]
                                attr:class="relative flex w-full touch-none select-none items-center"
                            >
                                <SliderTrack attr:class="relative h-2 w-full grow overflow-hidden rounded-full bg-slate-200">
                                    <SliderRange attr:class="absolute h-full bg-accent-600" />
                                </SliderTrack>
                                <SliderThumb attr:class="block h-5 w-5 rounded-full border-2 border-accent-600 bg-white shadow-sm ring-offset-white transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-400 focus-visible:ring-offset-2" />
                            </Slider>
                        </div>

                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Price Range"</label>
                            <Slider
                                default_value=vec![25.0, 75.0]
                                attr:class="relative flex w-full touch-none select-none items-center"
                            >
                                <SliderTrack attr:class="relative h-2 w-full grow overflow-hidden rounded-full bg-slate-200">
                                    <SliderRange attr:class="absolute h-full bg-accent-600" />
                                </SliderTrack>
                                <SliderThumb attr:class="block h-5 w-5 rounded-full border-2 border-accent-600 bg-white shadow-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-400 focus-visible:ring-offset-2" />
                                <SliderThumb attr:class="block h-5 w-5 rounded-full border-2 border-accent-600 bg-white shadow-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-400 focus-visible:ring-offset-2" />
                            </Slider>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::slider::*;

view! {
    <Slider default_value=vec![50.0]>
        <SliderTrack>
            <SliderRange />
        </SliderTrack>
        <SliderThumb />
    </Slider>
}"# />
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
