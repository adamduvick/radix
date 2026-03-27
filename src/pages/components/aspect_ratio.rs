use leptos::prelude::*;
use pith_ui::aspect_ratio::AspectRatio;

use crate::components::*;

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
                <DemoSection>
                    <div class="grid gap-4 sm:grid-cols-3 max-w-lg">
                        <div>
                            <p class="mb-2 text-xs font-medium text-slate-500">"16:9"</p>
                            <AspectRatio ratio=16.0 / 9.0>
                                <div class="flex h-full w-full items-center justify-center rounded-lg bg-accent-100 text-sm font-medium text-accent-700">
                                    "16:9"
                                </div>
                            </AspectRatio>
                        </div>
                        <div>
                            <p class="mb-2 text-xs font-medium text-slate-500">"1:1"</p>
                            <AspectRatio ratio=1.0>
                                <div class="flex h-full w-full items-center justify-center rounded-lg bg-emerald-100 text-sm font-medium text-emerald-700">
                                    "1:1"
                                </div>
                            </AspectRatio>
                        </div>
                        <div>
                            <p class="mb-2 text-xs font-medium text-slate-500">"4:3"</p>
                            <AspectRatio ratio=4.0 / 3.0>
                                <div class="flex h-full w-full items-center justify-center rounded-lg bg-amber-100 text-sm font-medium text-amber-700">
                                    "4:3"
                                </div>
                            </AspectRatio>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::aspect_ratio::AspectRatio;

view! {
    <div style="width: 300px">
        <AspectRatio ratio=16.0 / 9.0>
            <img src="photo.jpg" style="object-fit: cover; width: 100%; height: 100%;" />
        </AspectRatio>
    </div>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="AspectRatio" description="Enforces a width-to-height ratio on its child content. Defaults to 1:1." />
                </div>
            </DocSection>
        </div>
    }
}
