use super::component_prelude::*;
use super::entry::Props as EntryProps;
use json::JsonValue;

pub enum Msg {
    FetchData,
    ReceiveData(String),
    ReceiveError(StatusCode),
    ToggleCompleted(usize),
    None,
}

pub struct TodoApp {
    fetch_service:          FetchService,
    json_data:              Option<JsonValue>,
    fetch_error:            Option<StatusCode>,
    fetching_data:          bool,
    fetch_task:             Option<FetchTask>,
    toggle_entry_completed: Callback<usize>,
    link:                   ComponentLink<Self>,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let todo_app = TodoApp {
            fetch_service:          Default::default(),
            json_data:              None,
            fetch_error:            None,
            fetching_data:          false,
            fetch_task:             None,
            toggle_entry_completed: link
                .send_back(|id| Msg::ToggleCompleted(id)),
            link:                   link,
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
                if let Ok(json_data) = json::parse(data.as_str()) {
                    self.json_data = Some(json_data);
                } else {
                    console::error("Failed parsing data to JSON");
                }
                true
            }
            Msg::ReceiveError(status) => {
                self.fetching_data = false;
                self.fetch_error = Some(status);
                self.fetch_task = None;
                true
            }
            Msg::ToggleCompleted(id) => {
                self.send_toggle_entry_completed(id);
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

    fn send_toggle_entry_completed(&mut self, id: usize) {
        let req = Request::builder()
            .uri(format!("toggle_entry/{}", id).as_str())
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

impl Renderable<Self> for TodoApp {
    fn view(&self) -> Html<Self> {
        let entries: Vec<EntryProps> = if let Some(data) = &self.json_data {
            data["entries"]
                .members()
                .map(|entry| {
                    serde_json::from_str::<EntryProps>(
                        entry.to_string().as_str(),
                    )
                    .expect("Failed deserializing JSON string")
                })
                .collect()
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
                <EntryList
                 entries = entries
                 toggle_entry_completed = &self.toggle_entry_completed />
            </div>
        }
    }
}
