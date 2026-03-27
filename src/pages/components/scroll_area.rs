use leptos::prelude::*;
use pith_ui::scroll_area::*;

use crate::components::*;

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
                <DemoSection>
                    <ScrollArea
                        attr:class="h-48 w-full max-w-md rounded-lg border border-slate-200"
                        r#type=ScrollAreaType::Auto
                    >
                        <ScrollAreaViewport attr:class="h-full w-full p-4">
                            <div class="space-y-4">
                                {(1..=20).map(|i| view! {
                                    <div class="rounded-md border border-slate-100 bg-slate-50 px-3 py-2 text-sm text-slate-700">
                                        {format!("Item {i} — Lorem ipsum dolor sit amet, consectetur adipiscing elit.")}
                                    </div>
                                }).collect_view()}
                            </div>
                        </ScrollAreaViewport>
                        <ScrollAreaScrollbar
                            orientation=Orientation::Vertical
                            attr:class="flex w-2.5 touch-none select-none bg-transparent p-0.5 transition-colors"
                        >
                            <ScrollAreaThumb attr:class="relative flex-1 rounded-full bg-slate-300 hover:bg-slate-400 transition-colors">" "</ScrollAreaThumb>
                        </ScrollAreaScrollbar>
                    </ScrollArea>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::scroll_area::*;

view! {
    <ScrollArea r#type=ScrollAreaType::Auto>
        <ScrollAreaViewport attr:class="viewport">
            // Scrollable content here
        </ScrollAreaViewport>
        <ScrollAreaScrollbar orientation=Orientation::Vertical>
            <ScrollAreaThumb />
        </ScrollAreaScrollbar>
        <ScrollAreaScrollbar orientation=Orientation::Horizontal>
            <ScrollAreaThumb />
        </ScrollAreaScrollbar>
        <ScrollAreaCorner />
    </ScrollArea>
}"# />
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
