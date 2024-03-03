use yew::prelude::*;
use yew_router::prelude::*;

use crate::{layout::ThemeContext, router::Route};

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    let route = use_route::<Route>().unwrap();
    let theme = use_context::<ThemeContext>().unwrap();

    if route == props.link {
        theme.dispatch(props.name.to_lowercase().replace(" ", "-"));
    }

    html! {
        <li id={format!("sidebar-option-{}", props.name.to_lowercase().replace(" ", "-"))} class="sidebar-option">
            <Link<Route> classes={classes!("flex", "sidebar-option-link", {if route == props.link {"selected"} else {""}})} to={props.link.clone()}>
                {props.name.clone()}
                <div class="sidebar-option-overlay page-background-colour"></div>
            </Link<Route>>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    pub link: Route,
    pub name: AttrValue,
    pub selected: bool
}
