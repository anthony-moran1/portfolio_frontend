use yew::prelude::*;

use crate::layout::Layout;

#[function_component(NotImplemented)]
pub fn not_implemented(props: &NotImplementedProps) -> Html {
    html! {
        <Layout title={format!("{} - Not implemented", props.title)}>
            <p>{"Sorry, this page has not been implemented yet. However this website is always being updated, so come back soon and maybe they'll be something to see!"}</p>
        </Layout>
    }
}

#[derive(Properties, PartialEq)]
pub struct NotImplementedProps {
    pub title: AttrValue,
}
