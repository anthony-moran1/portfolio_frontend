use yew::prelude::*;

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let cls;
    if props.selected {
        cls = "selected"
    } else {
        cls = ""
    };

    html! {
        <li id={format!("sidebar-option-{}", props.name.to_lowercase().replace(" ", "-"))} class="sidebar-option">
            <a class={format!("flex sidebar-option-link {}", cls)} href={props.link.clone()}>{props.name.clone()}</a>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    pub link: String,
    pub name: String,
    pub selected: bool
}