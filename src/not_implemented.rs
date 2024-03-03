use yew::prelude::*;

#[function_component(NotImplemented)]
pub fn not_implemented(_props: &NotImplementedProps) -> Html {
    html! {
        <p>{"Sorry, this page has not been implemented yet. However this website is always being updated, so come back soon and maybe they'll be something to see!"}</p>
    }
}

#[derive(Properties, PartialEq)]
pub struct NotImplementedProps {
    pub title: AttrValue,
}
