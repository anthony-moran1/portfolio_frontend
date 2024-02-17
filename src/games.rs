use yew::prelude::*;
use crate::layout::Layout;

#[function_component(Games)]
pub fn games() -> Html {
    html! {
    <Layout title="Games">
        <p>{"I promise I do actually make games. You will be able to play them here soon!"}</p>
    </Layout>
    }
}
