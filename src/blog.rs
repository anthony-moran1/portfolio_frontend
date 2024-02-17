use yew::prelude::*;
use crate::layout::Layout;

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
    <Layout title="Blog">
        <p>{"I'm thinking of vlogging instead... What do you think?"}</p>
    </Layout>
    }
}
