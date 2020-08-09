#![recursion_limit = "512"]

use yew::web_sys::{console, Node};
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

const HTML: &str = include_str!("document.html");

struct Model {
    value: i64,
}

enum Msg {
    
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {value: 0}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let js_svg = {
            let div = yew::web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(HTML);
            console::log_1(&div);
            div
        };
        //eprintln!("js_svg: {:?}", js_svg);
        let node = Node::from(js_svg);
        let vnode = VNode::VRef(node);
        //eprintln!("svg: {:?}", vnode);
        vnode
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}