use leptos::prelude::*;
use pith_ui::tabs::*;

use crate::components::{extract_demo, DemoTabs};

const TAB_LIST: &str = "flex border-b border-slate-200";
const TAB_TRIGGER: &str = "border-b-2 border-transparent px-4 py-2.5 text-sm font-medium \
    text-slate-600 hover:text-slate-900 transition-colors \
    data-[state=active]:border-accent-600 data-[state=active]:text-accent-600";
const TAB_CONTENT: &str = "mt-4";

// #region demo
#[component]
pub fn TabsBasic() -> impl IntoView {
    view! {
        <Tabs default_value="account".to_string() attr:class="w-full max-w-md">
            <TabsList attr:class=TAB_LIST>
                <TabsTrigger value="account".to_string() attr:class=TAB_TRIGGER>
                    "Account"
                </TabsTrigger>
                <TabsTrigger value="password".to_string() attr:class=TAB_TRIGGER>
                    "Password"
                </TabsTrigger>
                <TabsTrigger value="notifications".to_string() attr:class=TAB_TRIGGER>
                    "Notifications"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="account".to_string() attr:class=TAB_CONTENT>
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
            <TabsContent value="password".to_string() attr:class=TAB_CONTENT>
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
            <TabsContent value="notifications".to_string() attr:class=TAB_CONTENT>
                <p class="text-sm text-slate-600">"Configure how you receive notifications."</p>
            </TabsContent>
        </Tabs>
    }
}
// #endregion demo

#[component]
pub fn TabsBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A tabbed interface with account, password, and notification panels."
        </p>

        <DemoTabs source=extract_demo(include_str!("tabs_basic.rs"))>
            <TabsBasic />
        </DemoTabs>
    }
}
