use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::layout::{DocsLayout, SiteHeader};
use crate::pages::{
    components::{
        AccessibleIconPage, AccordionPage, AlertDialogPage, AspectRatioPage, AvatarPage,
        CalendarPage, CheckboxPage, CollapsiblePage, ComboboxPage, ComponentsOverview,
        ContextMenuPage, DialogPage, DropdownMenuPage, FormPage, HoverCardPage, LabelPage,
        MenuPage, MenubarPage, NavigationMenuPage, OneTimePasswordFieldPage,
        PasswordToggleFieldPage, PopoverPage, ProgressPage, RadioGroupPage, ScrollAreaPage,
        SelectPage, SeparatorPage, SliderPage, SwitchPage, TabsPage, TimeFieldPage, ToastPage,
        ToggleGroupPage, TogglePage, ToolbarPage, TooltipPage,
    },
    getting_started::GettingStarted,
    home::Home,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <SiteHeader />
            <Routes fallback=|| view! {
                <div class="flex items-center justify-center min-h-[60vh]">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold text-slate-900 mb-4">"404"</h1>
                        <p class="text-slate-600">"Page not found."</p>
                    </div>
                </div>
            }>
                <Route path=path!("/") view=Home />
                <ParentRoute path=path!("/docs") view=DocsLayout>
                    <Route path=path!("/getting-started") view=GettingStarted />
                    <Route path=path!("/components") view=ComponentsOverview />
                    <Route path=path!("/components/accessible-icon") view=AccessibleIconPage />
                    <Route path=path!("/components/accordion") view=AccordionPage />
                    <Route path=path!("/components/alert-dialog") view=AlertDialogPage />
                    <Route path=path!("/components/aspect-ratio") view=AspectRatioPage />
                    <Route path=path!("/components/avatar") view=AvatarPage />
                    <Route path=path!("/components/calendar") view=CalendarPage />
                    <Route path=path!("/components/checkbox") view=CheckboxPage />
                    <Route path=path!("/components/collapsible") view=CollapsiblePage />
                    <Route path=path!("/components/combobox") view=ComboboxPage />
                    <Route path=path!("/components/context-menu") view=ContextMenuPage />
                    <Route path=path!("/components/dialog") view=DialogPage />
                    <Route path=path!("/components/dropdown-menu") view=DropdownMenuPage />
                    <Route path=path!("/components/form") view=FormPage />
                    <Route path=path!("/components/hover-card") view=HoverCardPage />
                    <Route path=path!("/components/label") view=LabelPage />
                    <Route path=path!("/components/menu") view=MenuPage />
                    <Route path=path!("/components/menubar") view=MenubarPage />
                    <Route path=path!("/components/navigation-menu") view=NavigationMenuPage />
                    <Route path=path!("/components/otp-field") view=OneTimePasswordFieldPage />
                    <Route path=path!("/components/password-toggle-field") view=PasswordToggleFieldPage />
                    <Route path=path!("/components/popover") view=PopoverPage />
                    <Route path=path!("/components/progress") view=ProgressPage />
                    <Route path=path!("/components/radio-group") view=RadioGroupPage />
                    <Route path=path!("/components/scroll-area") view=ScrollAreaPage />
                    <Route path=path!("/components/select") view=SelectPage />
                    <Route path=path!("/components/separator") view=SeparatorPage />
                    <Route path=path!("/components/slider") view=SliderPage />
                    <Route path=path!("/components/switch") view=SwitchPage />
                    <Route path=path!("/components/tabs") view=TabsPage />
                    <Route path=path!("/components/time-field") view=TimeFieldPage />
                    <Route path=path!("/components/toast") view=ToastPage />
                    <Route path=path!("/components/toggle") view=TogglePage />
                    <Route path=path!("/components/toggle-group") view=ToggleGroupPage />
                    <Route path=path!("/components/toolbar") view=ToolbarPage />
                    <Route path=path!("/components/tooltip") view=TooltipPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
