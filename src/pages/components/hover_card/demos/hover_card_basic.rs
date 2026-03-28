use leptos::prelude::*;
use pith_ui::hover_card::*;

use crate::components::{extract_demo, DemoTabs};

const TRIGGER: &str = "inline-flex items-center rounded-full bg-accent-50 px-3 py-1 \
    text-sm font-medium text-accent-700 underline decoration-accent-300 \
    hover:bg-accent-100 transition-colors cursor-pointer";
const CONTENT: &str = "popover-content w-64 rounded-xl border border-slate-200 \
    bg-white p-4 shadow-lg";
const AVATAR: &str = "flex h-10 w-10 items-center justify-center rounded-full \
    bg-accent-100 text-sm font-bold text-accent-700";
const ARROW: &str = "fill-white [filter:drop-shadow(0_1px_2px_rgb(0_0_0/0.06))]";

// #region demo
#[component]
pub fn HoverCardBasic() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <HoverCard>
                <HoverCardTrigger attr:class=TRIGGER>
                    "@pith-ui"
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent
                        attr:class=CONTENT
                        side_offset=5.0
                    >
                        <div class="space-y-2">
                            <div class=AVATAR>"P"</div>
                            <div>
                                <h4 class="text-sm font-semibold text-slate-900">"Pith UI"</h4>
                                <p class="text-xs text-slate-500">"@pith-ui"</p>
                            </div>
                            <p class="text-sm text-slate-600">
                                "Accessible UI primitives for Leptos. The core of your interface."
                            </p>
                        </div>
                        <HoverCardArrow attr:class=ARROW />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCard>
        </div>
    }
}
// #endregion demo

#[component]
pub fn HoverCardBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A hover card showing a user profile preview with avatar, name, and description when hovering over a mention."
        </p>

        <DemoTabs source=extract_demo(include_str!("hover_card_basic.rs"))>
            <HoverCardBasic />
        </DemoTabs>
    }
}
