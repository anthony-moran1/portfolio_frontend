use crate::sidebar_item::{SidebarItem, SidebarItemProps};
use web_sys::window;
use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let start_items = vec![SidebarItemProps {
        link: AttrValue::from("games"),
        name: AttrValue::from("Games"),
        selected: false,
    }];

    let main_items = vec![
        SidebarItemProps {
            link: AttrValue::from(""),
            name: AttrValue::from("Home"),
            selected: false,
        },
        SidebarItemProps {
            link: AttrValue::from("projects"),
            name: AttrValue::from("Projects"),
            selected: false,
        },
        SidebarItemProps {
            link: AttrValue::from("about_me"),
            name: AttrValue::from("About Me"),
            selected: false,
        },
    ];

    let end_items = vec![
        SidebarItemProps {
            link: AttrValue::from("socials"),
            name: AttrValue::from("Socials"),
            selected: false,
        },
        SidebarItemProps {
            link: AttrValue::from("blog"),
            name: AttrValue::from("Blog"),
            selected: false,
        },
        SidebarItemProps {
            link: AttrValue::from("settings"),
            name: AttrValue::from("Settings"),
            selected: false,
        },
    ];

    html! {
    <nav>
        {sidebar_items("sidebar-options-start".to_string(), start_items)}
        {sidebar_items("sidebar-options-main".to_string(), main_items)}
        {sidebar_items("sidebar-options-end".to_string(), end_items)}
    </nav>
    }
}

fn sidebar_items(id: String, sidebar_items: Vec<SidebarItemProps>) -> Html {
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
    <ul id={id} class="sidebar-options">
    { for sidebar_items.iter().map(|props| {
        let selected = current_pathname == props.link;
        html! { <SidebarItem link={props.link.clone()} name={props.name.clone()} selected={selected} /> }
    }) }
    </ul>
    }
}
