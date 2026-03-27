use leptos::prelude::*;
use pith_ui::separator::Separator;

use crate::components::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Separator"
                description="Visually or semantically separates content."
                features=&["Accessible", "Horizontal & vertical", "Decorative mode"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <div class="max-w-sm">
                        <div>
                            <h3 class="text-base font-semibold text-slate-900">"Pith UI"</h3>
                            <p class="text-sm text-slate-600">"An open-source component library."</p>
                        </div>
                        <Separator attr:class="my-4 h-px bg-slate-200" decorative=true />
                        <div class="flex h-5 items-center gap-4 text-sm text-slate-600">
                            <span>"Blog"</span>
                            <Separator
                                orientation=pith_ui::separator::Orientation::Vertical
                                attr:class="h-full w-px bg-slate-200"
                                decorative=true
                            />
                            <span>"Docs"</span>
                            <Separator
                                orientation=pith_ui::separator::Orientation::Vertical
                                attr:class="h-full w-px bg-slate-200"
                                decorative=true
                            />
                            <span>"Source"</span>
                        </div>
                    </div>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::separator::*;

view! {
    // Horizontal
    <Separator attr:class="h-px bg-slate-200" />

    // Vertical
    <Separator
        orientation=Orientation::Vertical
        attr:class="w-px bg-slate-200"
    />
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Separator" description="A horizontal or vertical line. When decorative=true, it is hidden from screen readers." />
                </div>
            </DocSection>
        </div>
    }
}
