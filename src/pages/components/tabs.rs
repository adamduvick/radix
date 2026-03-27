use leptos::prelude::*;
use pith_ui::tabs::*;

use crate::components::*;

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
                <DemoSection>
                    <Tabs default_value="account".to_string() attr:class="w-full max-w-md">
                        <TabsList attr:class="flex border-b border-slate-200">
                            <TabsTrigger
                                value="account".to_string()
                                attr:class="border-b-2 border-transparent px-4 py-2.5 text-sm font-medium text-slate-600 hover:text-slate-900 transition-colors data-[state=active]:border-accent-600 data-[state=active]:text-accent-600"
                            >
                                "Account"
                            </TabsTrigger>
                            <TabsTrigger
                                value="password".to_string()
                                attr:class="border-b-2 border-transparent px-4 py-2.5 text-sm font-medium text-slate-600 hover:text-slate-900 transition-colors data-[state=active]:border-accent-600 data-[state=active]:text-accent-600"
                            >
                                "Password"
                            </TabsTrigger>
                            <TabsTrigger
                                value="notifications".to_string()
                                attr:class="border-b-2 border-transparent px-4 py-2.5 text-sm font-medium text-slate-600 hover:text-slate-900 transition-colors data-[state=active]:border-accent-600 data-[state=active]:text-accent-600"
                            >
                                "Notifications"
                            </TabsTrigger>
                        </TabsList>
                        <TabsContent value="account".to_string() attr:class="mt-4">
                            <div class="space-y-4">
                                <p class="text-sm text-slate-600">"Manage your account settings and preferences."</p>
                                <div>
                                    <label class="block text-sm font-medium text-slate-700">"Display name"</label>
                                    <input
                                        type="text"
                                        value="Ada Lovelace"
                                        class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm"
                                    />
                                </div>
                            </div>
                        </TabsContent>
                        <TabsContent value="password".to_string() attr:class="mt-4">
                            <div class="space-y-4">
                                <p class="text-sm text-slate-600">"Change your password."</p>
                                <div>
                                    <label class="block text-sm font-medium text-slate-700">"Current password"</label>
                                    <input type="password" class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-slate-700">"New password"</label>
                                    <input type="password" class="mt-1 w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
                                </div>
                            </div>
                        </TabsContent>
                        <TabsContent value="notifications".to_string() attr:class="mt-4">
                            <p class="text-sm text-slate-600">"Configure how you receive notifications."</p>
                        </TabsContent>
                    </Tabs>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::tabs::*;

view! {
    <Tabs default_value="tab1".to_string()>
        <TabsList>
            <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
            <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
        </TabsList>
        <TabsContent value="tab1".to_string()>
            "Content 1"
        </TabsContent>
        <TabsContent value="tab2".to_string()>
            "Content 2"
        </TabsContent>
    </Tabs>
}"# />
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
