use super::component_prelude::*;
use json::JsonValue;

pub enum Msg {
    FetchData,
    ReceiveData(String),
    ReceiveError(StatusCode),
    None,
}

pub struct TodoApp {
    fetch_service: FetchService,
    json_data:     Option<JsonValue>,
    fetch_error:   Option<StatusCode>,
    fetching_data: bool,
    fetch_task:    Option<FetchTask>,
    link:          ComponentLink<Self>,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let todo_app = TodoApp {
            fetch_service: Default::default(),
            json_data:     None,
            fetch_error:   None,
            fetching_data: false,
            fetch_task:    None,
            link:          link,
        };
        todo_app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching_data = true;
                self.fetch_error = None;
                self.fetch_data();
                true
            }
            Msg::ReceiveData(data) => {
                self.fetching_data = false;
                self.fetch_task = None;
                true
            }
            Msg::ReceiveError(status) => {
                self.fetching_data = false;
                self.fetch_error = Some(status);
                self.fetch_task = None;
                true
            }
            Msg::None => false,
        }
    }
}

impl TodoApp {
    fn fetch_data(&mut self) {
        let req = Request::builder()
            .uri("todo_data")
            .method("GET")
            .body(format::Nothing)
            .unwrap();

        let callback =
            self.link
                .send_back(move |res: Response<Result<String, Error>>| {
                    let (meta, data) = res.into_parts();
                    if meta.status.is_success() {
                        Msg::ReceiveData(data.unwrap())
                    } else {
                        Msg::ReceiveError(meta.status)
                    }
                });

        self.fetch_task = Some(self.fetch_service.fetch(req, callback));
    }
}

impl Renderable<TodoApp> for TodoApp {
    fn view(&self) -> Html<Self> {
        let entries: Vec<String> = if let Some(data) = &self.json_data {
            data["entries"].members().map(ToString::to_string).collect()
        } else {
            Vec::new()
        };
        html! {
            <div>
                <p>
                    <button onclick=|_| Msg::FetchData>
                        { "Fetch Data" }
                    </button>
                    {
                        if self.fetching_data {
                            html! {
                                <em>
                                    { "Fetching..." }
                                </em>
                            }
                        } else {
                            if let Some(error_status) = self.fetch_error {
                                html! {
                                    <pre>
                                        { "Error fetching data" }
                                        <br />
                                        { "Status Code: " }
                                        <strong>{ error_status }</strong>
                                    </pre>
                                }
                            } else {
                                html! {}
                            }
                        }
                    }
                </p>
                <EntryList entries=entries />
            </div>
        }
    }
}
