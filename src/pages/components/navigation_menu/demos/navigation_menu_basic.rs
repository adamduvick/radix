use leptos::prelude::*;
use pith_ui::navigation_menu::*;

use crate::components::{extract_demo, DemoTabs};

const NAV_LIST: &str = "flex items-center gap-1 rounded-lg border border-slate-200 \
    bg-white p-1 shadow-sm";
const TRIGGER: &str = "rounded-md px-3 py-2 text-sm font-medium text-slate-700 \
    hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors";
const CONTENT: &str = "absolute left-0 top-full mt-1 w-[400px] rounded-lg border \
    border-slate-200 bg-white p-4 shadow-lg";
const LINK_CARD: &str = "block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors";
const LINK_INLINE: &str = "rounded-md px-3 py-2 text-sm font-medium text-slate-700 \
    no-underline hover:bg-slate-100 transition-colors";
const LINK_TITLE: &str = "text-sm font-medium text-slate-900";
const LINK_DESC: &str = "mt-1 text-xs text-slate-500";

// #region demo
#[component]
pub fn NavigationMenuBasic() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="relative">
            <NavigationMenuList attr:class=NAV_LIST>
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class=TRIGGER>
                        "Products"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class=CONTENT>
                        <div class="grid grid-cols-2 gap-3">
                            <NavigationMenuLink attr:class=LINK_CARD attr:href="#">
                                <div class=LINK_TITLE>"Analytics"</div>
                                <p class=LINK_DESC>"Track your performance metrics."</p>
                            </NavigationMenuLink>
                            <NavigationMenuLink attr:class=LINK_CARD attr:href="#">
                                <div class=LINK_TITLE>"Automation"</div>
                                <p class=LINK_DESC>"Streamline your workflow."</p>
                            </NavigationMenuLink>
                            <NavigationMenuLink attr:class=LINK_CARD attr:href="#">
                                <div class=LINK_TITLE>"Security"</div>
                                <p class=LINK_DESC>"Keep your data safe."</p>
                            </NavigationMenuLink>
                            <NavigationMenuLink attr:class=LINK_CARD attr:href="#">
                                <div class=LINK_TITLE>"Integrations"</div>
                                <p class=LINK_DESC>"Connect with your tools."</p>
                            </NavigationMenuLink>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class=LINK_INLINE attr:href="#">
                        "Documentation"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:class=LINK_INLINE attr:href="#">
                        "Pricing"
                    </NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
// #endregion demo

#[component]
pub fn NavigationMenuBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A navigation menu with an expandable Products panel and plain links."
        </p>

        <DemoTabs source=extract_demo(include_str!("navigation_menu_basic.rs"))>
            <NavigationMenuBasic />
        </DemoTabs>
    }
}
