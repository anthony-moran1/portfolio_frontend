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
        theme: String::new()
    });

    html! {
    <ContextProvider<ThemeContext> context={theme.clone()}>
    <div theme={theme.theme.clone()} class="flex vh-100">
        <div id="background" class="page-background-colour"></div>
        <LeftContent />
        <div id="main-content">
            <main>
                <h2 class="page-colour">{props.title.clone()}</h2>
                {props.children.clone()}
            </main>
        </div>
    </div>
    </ContextProvider<ThemeContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub title: AttrValue,
    pub children: Html,
}
