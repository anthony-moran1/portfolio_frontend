use yew::prelude::*;
use crate::layout::Layout;

#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
    <Layout>
        <h1>{"Portfolio"}</h1>
        <p>{"Take a look at these settings!"}</p>
        <p>{"Please make youself at home :)"}</p>
    </Layout>
    }
}
