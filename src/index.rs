use crate::layout::Layout;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
    <Layout title="Home">
        <p>{"Welcome to my portfolio site!"}</p>
        <p>{"Here is another line!"}</p>
    </Layout>
    }
}
