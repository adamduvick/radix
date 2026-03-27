use leptos::prelude::*;
use pith_ui::toolbar::*;

use crate::components::*;

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
                <DemoSection>
                    <Toolbar attr:class="flex items-center gap-1 rounded-lg border border-slate-200 bg-white p-1 shadow-sm">
                        <ToolbarToggleGroup r#type=pith_ui::toggle_group::ToggleGroupType::Single default_value=vec!["center".to_string()] attr:class="flex gap-0.5">
                            <ToolbarToggleItem
                                value="left".to_string()
                                attr:class="rounded-md px-2.5 py-1.5 text-sm text-slate-600 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            >
                                "Left"
                            </ToolbarToggleItem>
                            <ToolbarToggleItem
                                value="center".to_string()
                                attr:class="rounded-md px-2.5 py-1.5 text-sm text-slate-600 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            >
                                "Center"
                            </ToolbarToggleItem>
                            <ToolbarToggleItem
                                value="right".to_string()
                                attr:class="rounded-md px-2.5 py-1.5 text-sm text-slate-600 hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 data-[state=on]:text-accent-700"
                            >
                                "Right"
                            </ToolbarToggleItem>
                        </ToolbarToggleGroup>

                        <ToolbarSeparator attr:class="mx-1 h-5 w-px bg-slate-200" />

                        <ToolbarButton attr:class="rounded-md px-2.5 py-1.5 text-sm font-medium text-slate-600 hover:bg-slate-100 transition-colors">
                            "Bold"
                        </ToolbarButton>
                        <ToolbarButton attr:class="rounded-md px-2.5 py-1.5 text-sm italic text-slate-600 hover:bg-slate-100 transition-colors">
                            "Italic"
                        </ToolbarButton>

                        <ToolbarSeparator attr:class="mx-1 h-5 w-px bg-slate-200" />

                        <ToolbarLink attr:class="rounded-md px-2.5 py-1.5 text-sm text-accent-600 no-underline hover:bg-accent-50 transition-colors" attr:href="">
                            "Help"
                        </ToolbarLink>
                    </Toolbar>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::toolbar::*;

view! {
    <Toolbar>
        <ToolbarToggleGroup r#type=ToggleGroupType::Single>
            <ToolbarToggleItem value="left".to_string()>"Left"</ToolbarToggleItem>
            <ToolbarToggleItem value="center".to_string()>"Center"</ToolbarToggleItem>
        </ToolbarToggleGroup>
        <ToolbarSeparator />
        <ToolbarButton>"Bold"</ToolbarButton>
        <ToolbarLink attr:href="/help">"Help"</ToolbarLink>
    </Toolbar>
}"# />
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
