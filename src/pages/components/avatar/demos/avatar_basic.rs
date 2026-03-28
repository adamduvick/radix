use leptos::prelude::*;
use pith_ui::avatar::*;

use crate::components::{extract_demo, DemoTabs};

const AVATAR: &str = "relative flex h-12 w-12 shrink-0 overflow-hidden rounded-full";
const AVATAR_SM: &str = "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full";
const FALLBACK_ACCENT: &str = "flex h-full w-full items-center justify-center rounded-full \
    bg-accent-100 text-sm font-medium text-accent-700";
const FALLBACK_EMERALD: &str = "flex h-full w-full items-center justify-center rounded-full \
    bg-emerald-100 text-sm font-medium text-emerald-700";
const FALLBACK_AMBER: &str = "flex h-full w-full items-center justify-center rounded-full \
    bg-amber-100 text-sm font-medium text-amber-700";
const FALLBACK_SLATE: &str = "flex h-full w-full items-center justify-center rounded-full \
    bg-slate-200 text-xs font-medium text-slate-600";

// #region demo
#[component]
pub fn AvatarBasic() -> impl IntoView {
    view! {
        <div class="flex items-center gap-4">
            <Avatar attr:class=AVATAR>
                <AvatarFallback attr:class=FALLBACK_ACCENT>
                    "AL"
                </AvatarFallback>
            </Avatar>

            <Avatar attr:class=AVATAR>
                <AvatarFallback attr:class=FALLBACK_EMERALD>
                    "JD"
                </AvatarFallback>
            </Avatar>

            <Avatar attr:class=AVATAR>
                <AvatarFallback attr:class=FALLBACK_AMBER>
                    "MK"
                </AvatarFallback>
            </Avatar>

            <Avatar attr:class=AVATAR_SM>
                <AvatarFallback attr:class=FALLBACK_SLATE>
                    "+3"
                </AvatarFallback>
            </Avatar>
        </div>
    }
}
// #endregion demo

#[component]
pub fn AvatarBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "Avatars with initial-based fallbacks in different colors and sizes."
        </p>

        <DemoTabs source=extract_demo(include_str!("avatar_basic.rs"))>
            <AvatarBasic />
        </DemoTabs>
    }
}
