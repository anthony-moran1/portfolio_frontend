use crate::sidebar_item::{SidebarItem, SidebarItemProps};
use crate::router::Route;
use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let start_items = vec![SidebarItemProps {
        link: Route::Games,
        name: AttrValue::from("Games"),
        selected: false,
    }];

    let main_items = vec![
        SidebarItemProps {
            link: Route::Home,
            name: AttrValue::from("Home"),
            selected: false,
        },
        SidebarItemProps {
            link: Route::Projects,
            name: AttrValue::from("Projects"),
            selected: false,
        },
        SidebarItemProps {
            link: Route::AboutMe,
            name: AttrValue::from("About Me"),
            selected: false,
        },
    ];

    let end_items = vec![
        SidebarItemProps {
            link: Route::Socials,
            name: AttrValue::from("Socials"),
            selected: false,
        },
        SidebarItemProps {
            link: Route::Blog,
            name: AttrValue::from("Blog"),
            selected: false,
        },
        SidebarItemProps {
            link: Route::Settings,
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
    html! {
    <ul id={id} class="sidebar-options">
    { for sidebar_items.iter().map(|props| {
        html! { <SidebarItem link={props.link.clone()} name={props.name.clone()} selected={false} /> }
    }) }
    </ul>
    }
}
