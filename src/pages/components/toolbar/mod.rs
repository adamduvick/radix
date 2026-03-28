use leptos::prelude::*;

use crate::components::*;

mod demos;
use demos::ToolbarBasicSection;

#[component]
pub fn ToolbarPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Toolbar"
                description="A container for grouping buttons, links, and toggle controls."
                features=&["Accessible", "Roving focus", "Separator support", "Toggle groups", "Horizontal & vertical"]
            />

            <DocSection title="Demo">
                <ToolbarBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Toolbar" description="Root container with roving tabindex focus management." />
                    <PartItem name="ToolbarButton" description="A button within the toolbar." />
                    <PartItem name="ToolbarLink" description="A link within the toolbar." />
                    <PartItem name="ToolbarToggleGroup" description="A group of toggle buttons within the toolbar." />
                    <PartItem name="ToolbarToggleItem" description="A toggle button within a toolbar toggle group." />
                    <PartItem name="ToolbarSeparator" description="Visual separator between toolbar groups." />
                </div>
            </DocSection>
        </div>
    }
}
