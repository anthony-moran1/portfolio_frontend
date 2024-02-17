use crate::layout::Layout;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
    <Layout title="Projects">
    <div class="projects">
        <div class="project">
            <h3>{"This Website!"}</h3>
            <p>{"Yes, it's a bit meta but this website is the product of my own design! I have used rust from start to end, including this front end!"}</p>
        </div>

        <div class="project">
            <h3>{"Video Games"}</h3>
            <p>{"If the "}<a href="/games">{"tab"}</a>{" on the left didn't give it away already, I make games! None in recent times I must admit (although that could be subject to change). In the mean time, do feel free to check out the previous games I have made!"}</p>

            <img src="images/Memories.png" />
        </div>
    </div>
    </Layout>
    }
}
