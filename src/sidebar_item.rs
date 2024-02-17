use yew::prelude::*;

use crate::layout::ThemeContext;

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let cls;
    let theme = use_context::<ThemeContext>().unwrap();

    if props.selected {
        cls = "selected";
        theme.dispatch(props.name.to_lowercase().replace(" ", "-"));
    } else {
        cls = "";
    };

    html! {
        <li id={format!("sidebar-option-{}", props.name.to_lowercase().replace(" ", "-"))} class="sidebar-option">
            <a class={format!("flex sidebar-option-link {}", cls)} href={props.link.clone()}>{props.name.clone()}</a>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    pub link: AttrValue,
    pub name: AttrValue,
    pub selected: bool
}
