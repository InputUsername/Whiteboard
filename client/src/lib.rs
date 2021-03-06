mod connect;
mod draw;

use connect::ConnectArea;
use draw::DrawArea;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
use yew::format::{Text, Binary};

use std::convert::From;

struct Model {
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
}

enum WsUpdate {
    Message,
    Opened,
    Closed,
    Error,
}

enum Msg {
    Connect(String),
    Disconnect,
    WsUpdate(WsUpdate),
}

fn alert(s: &str) {
    web_sys::window()
        .expect("window not available")
        .alert_with_message(s)
        .expect("alert failed");
}

struct IgnoreData;

impl From<Text> for IgnoreData {
    fn from(_data: Text) -> Self {
        IgnoreData
    }
}

impl From<Binary> for IgnoreData {
    fn from(_data: Binary) -> Self {
        IgnoreData
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect(addr) => {
                let callback = self.link.callback(|_data: IgnoreData| Msg::WsUpdate(WsUpdate::Message));
                let notification = self.link.callback(|status| match status {
                    WebSocketStatus::Opened => Msg::WsUpdate(WsUpdate::Opened),
                    WebSocketStatus::Closed => Msg::WsUpdate(WsUpdate::Closed),
                    WebSocketStatus::Error => Msg::WsUpdate(WsUpdate::Error),
                });
                let task = WebSocketService::connect_binary(&format!("ws://{}", addr), callback, notification)
                    .unwrap();
                self.ws = Some(task);
            }
            Msg::Disconnect => self.ws = None,
            Msg::WsUpdate(update) => match update {
                WsUpdate::Message => alert("Message received"),
                WsUpdate::Opened => alert("Opened"),
                WsUpdate::Closed => {
                    alert("Closed");
                    self.ws = None;
                }
                WsUpdate::Error => {
                    alert("Error");
                    self.ws = None;
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let connected = self.ws.is_some();
        let draw_area_if_connected = || -> Html {
            if connected {
                html! {
                    <DrawArea width=1600f64 height=900f64 />
                }
            } else {
                html! {}
            }
        };
        html! {
            <>
                <ConnectArea connected=connected
                    on_connect=self.link.callback(|addr| Msg::Connect(addr))
                    on_disconnect=self.link.callback(|_| Msg::Disconnect) />
                { draw_area_if_connected() }
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();

    App::<Model>::new().mount_to_body();
}