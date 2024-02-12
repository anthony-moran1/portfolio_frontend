use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <h1>{"Portfolio"}</h1>
        <p>{"Welcome to my portfolio site!"}</p>
        <p>{"Here is another line!"}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
