use leptos::prelude::*;
use pith_ui::toolbar::*;

use crate::components::{extract_demo, DemoTabs};

const TOOLBAR: &str = "flex items-center gap-1 rounded-lg border border-slate-200 \
    bg-white p-1 shadow-sm";
const TOGGLE_GROUP: &str = "flex gap-0.5";
const TOGGLE_ITEM: &str = "rounded-md px-2.5 py-1.5 text-sm text-slate-600 \
    hover:bg-slate-100 transition-colors data-[state=on]:bg-accent-100 \
    data-[state=on]:text-accent-700";
const BUTTON: &str = "rounded-md px-2.5 py-1.5 text-sm font-medium text-slate-600 \
    hover:bg-slate-100 transition-colors";
const BUTTON_ITALIC: &str = "rounded-md px-2.5 py-1.5 text-sm italic text-slate-600 \
    hover:bg-slate-100 transition-colors";
const SEPARATOR: &str = "mx-1 h-5 w-px bg-slate-200";
const LINK: &str = "rounded-md px-2.5 py-1.5 text-sm text-accent-600 no-underline \
    hover:bg-accent-50 transition-colors";

// #region demo
#[component]
pub fn ToolbarBasic() -> impl IntoView {
    view! {
        <Toolbar attr:class=TOOLBAR>
            <ToolbarToggleGroup
                r#type=pith_ui::toggle_group::ToggleGroupType::Single
                default_value=vec!["center".to_string()]
                attr:class=TOGGLE_GROUP
            >
                <ToolbarToggleItem
                    value="left".to_string()
                    attr:class=TOGGLE_ITEM
                >
                    "Left"
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    value="center".to_string()
                    attr:class=TOGGLE_ITEM
                >
                    "Center"
                </ToolbarToggleItem>
                <ToolbarToggleItem
                    value="right".to_string()
                    attr:class=TOGGLE_ITEM
                >
                    "Right"
                </ToolbarToggleItem>
            </ToolbarToggleGroup>

            <ToolbarSeparator attr:class=SEPARATOR />

            <ToolbarButton attr:class=BUTTON>
                "Bold"
            </ToolbarButton>
            <ToolbarButton attr:class=BUTTON_ITALIC>
                "Italic"
            </ToolbarButton>

            <ToolbarSeparator attr:class=SEPARATOR />

            <ToolbarLink attr:class=LINK attr:href="">
                "Help"
            </ToolbarLink>
        </Toolbar>
    }
}
// #endregion demo

#[component]
pub fn ToolbarBasicSection() -> impl IntoView {
    view! {
        <h3 class="mb-3 text-lg font-semibold text-slate-900">"Basic"</h3>
        <p class="mb-4 text-slate-600">
            "A toolbar with a toggle group, action buttons, a link, and separators between groups."
        </p>

        <DemoTabs source=extract_demo(include_str!("toolbar_basic.rs"))>
            <ToolbarBasic />
        </DemoTabs>
    }
}
