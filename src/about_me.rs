use yew::prelude::*;

use crate::layout::Layout;

#[function_component(AboutMe)]
pub fn about_me() -> Html {
    html! {
        <Layout title="About Me">
            <p>{"Oh... You want to know more about me?"}</p>
        </Layout>
    }
}