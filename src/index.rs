use crate::layout::Layout;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let children_middle = html! {
    <div class="flex align-items-center">
        <img src="images/arrows.png" />
        <p>{"Use "}<b class="page-colour">{"this"}</b>{" pull tab to see other pages!"}</p>
    </div>
    };

    html! {
    <Layout title="Home" {children_middle}>
        <p>{"Welcome to my portfolio site!"}</p>
        <p>{"Here is another line!"}</p>
    </Layout>
    }
}
