use std::rc::Rc;

use crate::left_content::LeftContent;
use yew::prelude::*;

/// reducer's State
#[derive(Debug, Eq, PartialEq, Clone, Properties)]
pub struct Theme {
    pub theme: String,
}

impl Reducible for Theme {
    type Action = String;

    /// Reducer Function
    fn reduce(self: Rc<Self>, theme: Self::Action) -> Rc<Self> {
        Self { theme }.into()
    }
}

pub type ThemeContext = UseReducerHandle<Theme>;

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let theme = use_reducer_eq(|| Theme {
        theme: String::new(),
    });

    html! {
    <ContextProvider<ThemeContext> context={theme.clone()}>
    <div theme={theme.theme.clone()}>
            <LeftContent />
            <main>
                <div id="main-start">
                    <h2 class="page-colour">{"Title"}</h2>
                    {props.children.clone()}
                </div>
            </main>
        </div>
    </ContextProvider<ThemeContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}
