use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let _children_middle = html! {
    <div class="flex align-items-center">
        <img src="static/images/arrows.png" />
        <p>{"Use "}<b class="page-colour">{"this"}</b>{" pull tab to see other pages!"}</p>
    </div>
    };

    html! {
        <p>{"Welcome to my portfolio site!"}</p>
    }
}
