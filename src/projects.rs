use yew::prelude::*;
use crate::layout::Layout;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
    <Layout>
        <h1>{"Portfolio"}</h1>
        <p>{"Here are the things I have made"}</p>
    </Layout>
    }
}
