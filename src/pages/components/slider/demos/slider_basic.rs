use leptos::prelude::*;
use pith_ui::slider::*;

use crate::components::{extract_demo, DemoTabs};

const SLIDER: &str = "relative flex w-full touch-none select-none items-center";
const TRACK: &str = "relative h-2 w-full grow overflow-hidden rounded-full bg-slate-200";
const RANGE: &str = "absolute h-full bg-accent-600";
const THUMB: &str = "block h-5 w-5 rounded-full border-2 border-accent-600 bg-white \
    shadow-sm ring-offset-white transition-colors focus-visible:outline-none \
    focus-visible:ring-2 focus-visible:ring-accent-400 focus-visible:ring-offset-2";

// #region demo
#[component]
pub fn SliderBasic() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-md">
            <div>
                <label class="mb-2 block text-sm font-medium text-slate-700">"Volume"</label>
                <Slider
                    default_value=vec![50.0]
                    attr:class=SLIDER
                >
                    <SliderTrack attr:class=TRACK>
                        <SliderRange attr:class=RANGE />
                    </SliderTrack>
                    <SliderThumb attr:class=THUMB />
                </Slider>
            </div>

            <div>
                <label class="mb-2 block text-sm font-medium text-slate-700">"Price Range"</label>
                <Slider
                    default_value=vec![25.0, 75.0]
                    attr:class=SLIDER
                >
                    <SliderTrack attr:class=TRACK>
                        <SliderRange attr:class=RANGE />
                    </SliderTrack>
                    <SliderThumb attr:class=THUMB />
                    <SliderThumb attr:class=THUMB />
                </Slider>
            </div>
        </div>
    }
}
// #endregion demo

#[component]
pub fn SliderBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A single-thumb slider for picking a value and a dual-thumb range slider for selecting a range."
        </p>

        <DemoTabs source=extract_demo(include_str!("slider_basic.rs"))>
            <SliderBasic />
        </DemoTabs>
    }
}
