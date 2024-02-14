use yew::prelude::*;
use crate::sidebar::Sidebar;

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
    <>
        <div class="vh-100 flex">
        <Sidebar />
        <main>
        {props.children.clone()}
        </main>
        </div>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html
}