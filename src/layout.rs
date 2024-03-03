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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Title(AttrValue);

impl Reducible for Title {
    type Action = AttrValue;

    fn reduce(self: Rc<Self>, title: Self::Action) -> Rc<Self> {
        Self(title).into()
    }
}

pub type ThemeContext = UseReducerHandle<Theme>;
pub type TitleContext = UseReducerHandle<Title>;

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let theme = use_reducer_eq(|| Theme {
        theme: String::new(),
    });

    let title = use_reducer_eq(|| Title(AttrValue::default()));

    html! {
    <ContextProvider<ThemeContext> context={theme.clone()}>
    <ContextProvider<TitleContext> context={title.clone()}>
        <div theme={theme.theme.clone()}>
            <LeftContent />
            <main>
                <h2 class="page-colour">{title.clone().0.to_string()}</h2>
                {props.children.clone()}
            </main>
        </div>
    </ContextProvider<TitleContext>>
    </ContextProvider<ThemeContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Html,
}
