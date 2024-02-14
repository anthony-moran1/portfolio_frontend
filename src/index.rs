use yew::prelude::*;
use crate::layout::Layout;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
    <Layout>
        <h1>{"Portfolio"}</h1>
        <p>{"Welcome to my portfolio site!"}</p>
        <p>{"Here is another line!"}</p>
    </Layout>
    }
}
