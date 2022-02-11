use gloo_events::EventListener;
use gloo_utils::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Vimeon"}</h2>
            <EditorLines />
        </div>
    }
}

pub enum Msg {
    Keydown(String),
}

pub struct EditorLines {
    lines: Vec<String>,
    keydown_listener: Option<EventListener>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditorLineProps {
    lines: Vec<String>,
}

impl Component for EditorLines {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keydown_listener: None,
            lines: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Keydown(key) => {
                self.lines.push(key);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <div>
            {
              self.lines.clone().into_iter().map(|line| {
                html!{<p>{line}</p>}
              }).collect::<Html>()
            }
          </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let on_keydown = ctx
            .link()
            .callback(|event: web_sys::KeyboardEvent| Msg::Keydown(event.key()));

        let element = gloo_utils::body();
        self.keydown_listener = Some(EventListener::new(&element, "keydown", move |e| {
            let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            on_keydown.emit(event.clone())
        }));
    }
}
