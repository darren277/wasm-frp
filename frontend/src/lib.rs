use serde::Deserialize;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    // Define the data structure you expect from the backend
    message: String,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

pub struct Model {
    fetch_task: Option<FetchTask>, // Task handle for the request
    data: Option<Data>, // Loaded data
    link: ComponentLink<Self>,
    error: Option<String>, // Error info
}

pub enum Msg {
    FetchData,
    ReceiveResponse(Result<Data, anyhow::Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchData); // Automatically fetch data when the component is created
        Self { 
            fetch_task: None, 
            data: None, 
            link, 
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                let request = Request::get("http://localhost:3000/data_endpoint")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Data, anyhow::Error>>>| {
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
                    Ok(data) => self.data = Some(data),
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None; // Clear the task now that the request is finished
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_data() }
                { self.view_error() }
            </>
        }
    }
}

impl Model {
    fn view_data(&self) -> Html {
        if let Some(ref data) = self.data {
            html! { <p>{ &data.message }</p> }
        } else {
            html! { <p>{ "No data yet..." }</p> }
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

fn main() {
    yew::start_app::<Model>();
}
