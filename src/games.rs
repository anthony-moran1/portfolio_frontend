use crate::layout::Layout;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

const WASM: &[u8] = include_bytes!("../games/first/mygame_bg.wasm");

#[function_component(Games)]
pub fn games() -> Html {
    let children_middle = html! {
        <canvas id="canvas-game" width="100px" style="width: 100px"></canvas>
    };

    html! {
        <Layout title="Games" {children_middle}>
            <p>{"I promise I do actually make games. You will be able to play them here soon!"}</p>
            <button onclick={Callback::from(|_| init())}>{"Press this button and maybe it will appear..."}</button>
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
