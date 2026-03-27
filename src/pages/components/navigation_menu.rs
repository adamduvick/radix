use leptos::prelude::*;
use pith_ui::navigation_menu::*;

use crate::components::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    view! {
        <div class="max-w-3xl">
            <ComponentPageHeader
                title="Navigation Menu"
                description="A collection of links for navigating websites, with support for expandable content panels."
                features=&["Accessible", "Keyboard navigation", "Submenus with viewport", "Active indicator", "RTL support"]
            />

            <DocSection title="Demo">
                <DemoSection>
                    <NavigationMenu attr:class="relative">
                        <NavigationMenuList attr:class="flex items-center gap-1 rounded-lg border border-slate-200 bg-white p-1 shadow-sm">
                            <NavigationMenuItem>
                                <NavigationMenuTrigger attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100 data-[state=open]:bg-slate-100 transition-colors">
                                    "Products"
                                </NavigationMenuTrigger>
                                <NavigationMenuContent attr:class="absolute left-0 top-full mt-1 w-[400px] rounded-lg border border-slate-200 bg-white p-4 shadow-lg">
                                    <div class="grid grid-cols-2 gap-3">
                                        <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                            <div class="text-sm font-medium text-slate-900">"Analytics"</div>
                                            <p class="mt-1 text-xs text-slate-500">"Track your performance metrics."</p>
                                        </NavigationMenuLink>
                                        <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                            <div class="text-sm font-medium text-slate-900">"Automation"</div>
                                            <p class="mt-1 text-xs text-slate-500">"Streamline your workflow."</p>
                                        </NavigationMenuLink>
                                        <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                            <div class="text-sm font-medium text-slate-900">"Security"</div>
                                            <p class="mt-1 text-xs text-slate-500">"Keep your data safe."</p>
                                        </NavigationMenuLink>
                                        <NavigationMenuLink attr:class="block rounded-md p-3 no-underline hover:bg-slate-50 transition-colors" attr:href="#">
                                            <div class="text-sm font-medium text-slate-900">"Integrations"</div>
                                            <p class="mt-1 text-xs text-slate-500">"Connect with your tools."</p>
                                        </NavigationMenuLink>
                                    </div>
                                </NavigationMenuContent>
                            </NavigationMenuItem>

                            <NavigationMenuItem>
                                <NavigationMenuLink attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 no-underline hover:bg-slate-100 transition-colors" attr:href="#">
                                    "Documentation"
                                </NavigationMenuLink>
                            </NavigationMenuItem>

                            <NavigationMenuItem>
                                <NavigationMenuLink attr:class="rounded-md px-3 py-2 text-sm font-medium text-slate-700 no-underline hover:bg-slate-100 transition-colors" attr:href="#">
                                    "Pricing"
                                </NavigationMenuLink>
                            </NavigationMenuItem>
                        </NavigationMenuList>
                    </NavigationMenu>
                </DemoSection>
            </DocSection>

            <DocSection title="Usage">
                <CodeBlock code=r#"use pith_ui::navigation_menu::*;

view! {
    <NavigationMenu>
        <NavigationMenuList>
            <NavigationMenuItem>
                <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                <NavigationMenuContent>
                    <NavigationMenuLink attr:href="/analytics">
                        "Analytics"
                    </NavigationMenuLink>
                </NavigationMenuContent>
            </NavigationMenuItem>
            <NavigationMenuItem>
                <NavigationMenuLink attr:href="/docs">"Docs"</NavigationMenuLink>
            </NavigationMenuItem>
        </NavigationMenuList>
    </NavigationMenu>
}"# />
            </DocSection>

            <DocSection title="API Reference">
                <div class="divide-y divide-slate-100">
                    <PartItem name="NavigationMenu" description="Root component. Manages open state." />
                    <PartItem name="NavigationMenuList" description="Container for menu items." />
                    <PartItem name="NavigationMenuItem" description="A top-level item (expandable or link)." />
                    <PartItem name="NavigationMenuTrigger" description="Button that opens content panel." />
                    <PartItem name="NavigationMenuContent" description="The expandable content panel." />
                    <PartItem name="NavigationMenuLink" description="A navigation link." />
                    <PartItem name="NavigationMenuSub" description="A sub-navigation for secondary lists." />
                    <PartItem name="NavigationMenuIndicator" description="Active indicator under the trigger." />
                    <PartItem name="NavigationMenuViewport" description="Shared viewport for content panels." />
                </div>
            </DocSection>
        </div>
    }
}
