use leptos::prelude::*;

use crate::components::*;
mod demos;
use demos::TabsBasicSection;

#[component]
pub fn TabsPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Tabs"
                description="A set of layered sections of content, known as tab panels, displayed one at a time."
                features=&["Accessible", "Keyboard navigation", "Automatic & manual activation", "Horizontal & vertical"]
            />

            <DocSection title="Demo">
                <TabsBasicSection />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="Tabs" description="Root component. Manages the active tab state." />
                    <PartItem name="TabsList" description="Container for the tab triggers." />
                    <PartItem name="TabsTrigger" description="Button that activates its associated content." />
                    <PartItem name="TabsContent" description="Content associated with a tab trigger." />
                </div>
            </DocSection>
        </div>
    }
}
