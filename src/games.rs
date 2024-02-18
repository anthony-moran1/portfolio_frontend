use crate::layout::Layout;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

const WASM: &[u8] = include_bytes!("../games/first/mygame_bg.wasm");

#[function_component(Games)]
pub fn games() -> Html {
    let canvas_visibility = use_state(|| "none".to_string());
    let button_visibility = use_state(|| "block".to_string());

    let onclick = {
        let canvas_visibility = canvas_visibility.clone();
        let button_visibility = button_visibility.clone();
        Callback::from(move |_| {
            canvas_visibility.set("block".to_string());
            button_visibility.set("none".to_string());
            init();
        })
    };

    let children_middle = html! {
        <>
            <button {onclick} style={format!("display: {}", *button_visibility)}>{"Press this button and maybe it will appear..."}</button>
            <canvas id="canvas-game" style={format!("display: {}", *canvas_visibility)}></canvas>
        </>
    };

    html! {
        <Layout title="Games" {children_middle}>
            <p>{"I promise I do actually make games. You will be able to play them here soon!"}</p>
        </Layout>
    }
}

fn init() {
    initSyncGame(WASM);
}

#[wasm_bindgen(module = "/games/first/mygame.js")]
extern "C" {
    fn initSyncGame(module: &[u8]);
}
