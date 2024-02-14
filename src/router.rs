use yew::prelude::*;
use yew_router::prelude::*;

use crate::index::Index;
use crate::projects::Projects;
use crate::about_me::AboutMe;

use crate::settings::Settings;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/about_me")]
    AboutMe,
    
    #[at("/settings")]
    Settings,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Index /> },
        Route::Projects => html! { <Projects /> },
        Route::AboutMe => html! { <AboutMe /> },

        Route::Settings => html! { <Settings /> },
        
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
