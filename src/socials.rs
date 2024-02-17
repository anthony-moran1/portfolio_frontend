use crate::layout::Layout;
use yew::prelude::*;

#[function_component(Socials)]
pub fn socials() -> Html {
    html! {
    <Layout title="Socials">
        <p>{"Check out my socials!"}</p>
    </Layout>
    }
}
