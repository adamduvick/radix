use leptos::prelude::*;
use pith_ui::separator::Separator;

use crate::components::{CodeBlock, DocSection};

#[component]
pub fn GettingStarted() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <div class="mb-10">
                <h1 class="text-3xl font-bold tracking-tight text-slate-900">"Getting Started"</h1>
                <p class="mt-2 text-lg text-slate-600">
                    "Install Pith UI and start building accessible interfaces with Leptos."
                </p>
                <Separator attr:class="mt-6 h-px bg-slate-200" decorative=true />
            </div>

            <DocSection title="Installation">
                <p class="mb-4 text-slate-600">
                    "Add Pith UI to your Leptos project. Enable only the components you need, or use the "
                    <code class="rounded bg-slate-100 px-1.5 py-0.5 text-sm font-mono text-accent-700">"all"</code>
                    " feature for everything."
                </p>
                <CodeBlock code=r#"# Cargo.toml
[dependencies]
pith-ui = { version = "0.0.2", features = ["dialog", "tabs", "checkbox"] }

# Or enable everything:
pith-ui = { version = "0.0.2", features = ["all"] }"# />
            </DocSection>

            <DocSection title="Quick Example">
                <p class="mb-4 text-slate-600">
                    "Components are unstyled by default. Add your own classes with "
                    <code class="rounded bg-slate-100 px-1.5 py-0.5 text-sm font-mono text-accent-700">"attr:class"</code>
                    "."
                </p>
                <CodeBlock code=r#"use leptos::prelude::*;
use pith_ui::dialog::*;

#[component]
fn MyDialog() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class="btn">
                "Open"
            </DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class="overlay" />
                <DialogContent attr:class="modal">
                    <DialogTitle>"Edit Profile"</DialogTitle>
                    <DialogDescription>
                        "Make changes to your profile."
                    </DialogDescription>
                    <DialogClose attr:class="btn">
                        "Save"
                    </DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}"# />
            </DocSection>

            <DocSection title="Feature Flags">
                <p class="mb-4 text-slate-600">
                    "Each component is behind a feature flag to keep compile times fast. Enable only what you need."
                </p>
                <div class="overflow-x-auto rounded-lg border border-slate-200">
                    <table class="min-w-full text-sm">
                        <thead>
                            <tr class="border-b border-slate-200 bg-slate-50">
                                <th class="px-4 py-2 text-left font-semibold text-slate-700">"Feature"</th>
                                <th class="px-4 py-2 text-left font-semibold text-slate-700">"Components"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-200">
                            <FeatureRow feature="accordion" components="Accordion, AccordionItem, AccordionTrigger, ..." />
                            <FeatureRow feature="dialog" components="Dialog, DialogTrigger, DialogContent, ..." />
                            <FeatureRow feature="tabs" components="Tabs, TabsList, TabsTrigger, TabsContent" />
                            <FeatureRow feature="checkbox" components="Checkbox, CheckboxIndicator" />
                            <FeatureRow feature="select" components="Select, SelectTrigger, SelectContent, ..." />
                            <FeatureRow feature="tooltip" components="TooltipProvider, Tooltip, TooltipTrigger, ..." />
                            <FeatureRow feature="all" components="All components" />
                        </tbody>
                    </table>
                </div>
            </DocSection>

            <DocSection title="Styling">
                <p class="mb-4 text-slate-600">
                    "Pith UI components are completely unstyled. Pass classes through "
                    <code class="rounded bg-slate-100 px-1.5 py-0.5 text-sm font-mono text-accent-700">"attr:class"</code>
                    " and use any CSS approach: Tailwind, CSS modules, plain CSS, or CSS-in-Rust."
                </p>
                <p class="text-slate-600">
                    "Components expose data attributes like "
                    <code class="rounded bg-slate-100 px-1.5 py-0.5 text-sm font-mono text-accent-700">"data-state=\"open\""</code>
                    " and "
                    <code class="rounded bg-slate-100 px-1.5 py-0.5 text-sm font-mono text-accent-700">"data-disabled"</code>
                    " for state-based styling."
                </p>
            </DocSection>
        </div>
    }
}

#[component]
fn FeatureRow(feature: &'static str, components: &'static str) -> impl IntoView {
    view! {
        <tr>
            <td class="px-4 py-2">
                <code class="rounded bg-slate-100 px-1.5 py-0.5 text-xs font-mono text-accent-700">{feature}</code>
            </td>
            <td class="px-4 py-2 text-slate-600">{components}</td>
        </tr>
    }
}
