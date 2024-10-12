use serde::Deserialize;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use anyhow;
use wasm_bindgen::prelude::*;
use web_sys::console;

// Macro to make logging easier
macro_rules! log {
    ($($t:tt)*) => (console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*))));
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    name: String,
}

pub struct Model {
    fetch_task: Option<FetchTask>,
    users: Option<Vec<User>>, // Expecting an array of users
    link: ComponentLink<Self>,
    error: Option<String>,
}

pub enum Msg {
    FetchData,
    ReceiveResponse(Result<Vec<User>, anyhow::Error>), // Expecting an array of users
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchData); // Automatically fetch data when the component is created
        Self { 
            fetch_task: None, 
            users: None,  // Expect an array of users
            link, 
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                let request = Request::get("http://127.0.0.1:8080/api/data")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<User>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(users) => {
                        log!("Users received: {:?}", users); // Log the data
                        self.users = Some(users);
                    }
                    Err(error) => {
                        log!("Error: {:?}", error);
                        self.error = Some(error.to_string());
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_users() }
                { self.view_error() }
            </>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Model {
    fn view_users(&self) -> Html {
        if let Some(ref users) = self.users {
            html! {
                <ul>
                    { for users.iter().map(|user| html! { <li>{ &user.name }</li> }) }
                </ul>
            }
        } else {
            html! { <p>{ "No users found..." }</p> }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error }</p> }
        } else {
            html! { <></> }
        }
    }
}

#[wasm_bindgen]
pub fn your_exported_function() {
    //"Hello from WebAssembly!".to_string()
    yew::start_app::<Model>();
}
