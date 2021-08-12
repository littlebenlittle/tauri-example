#![recursion_limit = "1024"]

use js_sys;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use wasm_logger;
use yew::prelude::*;

#[wasm_bindgen(
    inline_js = "export function invoke_tauri(cmd, args = {}) { return window.__TAURI__._invoke(cmd, args=args) }"
)]
extern "C" {
    async fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_answer() -> String {
    let answer = invoke_tauri("get_answer", JsValue::undefined()).await;
    match answer.as_string() {
        Some(s) => s,
        None => "unknown!".into(),
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    Ok(())
}

enum Message {
    RequestAnswer,
    UpdateAnswer(String),
}

#[derive(Clone, PartialEq)]
struct Properties {
    pub answer: String,
}

struct Model {
    props: Properties,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            props: Properties {
                answer: "".to_string(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::RequestAnswer => {
                let cb = self.link.callback(Message::UpdateAnswer);
                wasm_bindgen_futures::spawn_local(async move {
                    let answer = get_answer().await;
                    cb.emit(answer)
                });
                false
            }
            Message::UpdateAnswer(answer) => {
                self.props.answer = answer;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <button onclick=self.link.callback(|_| Message::RequestAnswer )>{"Request Answer"}</button>
            <p>{format!("Answer: {}", self.props.answer)}</p>
            </>
        }
    }
}
