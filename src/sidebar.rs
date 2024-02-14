use crate::sidebar_item::{SidebarItem, SidebarItemProps};
use yew::prelude::*;
use web_sys::window;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let mut items = vec![
        SidebarItemProps {
            link: "/".to_string(),
            name: "Home".to_string(),
            selected: false
        },
        SidebarItemProps {
            link: "/projects".to_string(),
            name: "Projects".to_string(),
            selected: false
        },
        SidebarItemProps {
            link: "/about_me".to_string(),
            name: "About me".to_string(),
            selected: false
        },
    ];

    let mut current_pathname = String::new();

    if let Some(window) = window() {
        // Get the location from the window object
        let location = window.location();

        // Get the href from the location object
        if let Ok(pathname) = location.pathname() {
            current_pathname = pathname;
        }
    }

    html! {
    <nav>
        <ul id="sidebar-options-main" class="sidebar-options">
            { for items.iter_mut().map(|props| {
                let selected = current_pathname == props.link;   

                html! { <SidebarItem link={props.link.clone()} name={props.name.clone()} selected={selected} /> }
            }) }
        </ul>

        <ul id="sidebar-options-end" class="sidebar-options">
            <SidebarItem link="/settings" name="Settings" selected={current_pathname == "/settings"} />
        </ul>
    </nav>
    }
}
