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
                    <h2 class="page-colour">{props.title.clone()}</h2>
                    {props.children.clone()}
                </div>

                <div id="main-middle">
                    {props.children_middle.clone()}
                </div>

                <div id="main-end">
                    {props.children_end.clone()}
                </div>
            </main>
        </div>
    </ContextProvider<ThemeContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub title: AttrValue,
    pub children: Html,
    #[prop_or_default]
    pub children_middle: Html,
    #[prop_or_default]
    pub children_end: Html
}
