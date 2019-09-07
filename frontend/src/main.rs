extern crate json;
#[macro_use]
extern crate yew;

mod components;

use json::JsonValue;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use components::prelude::*;

const MOCKUP_DATA: &str = r#"
    {
        "entries": [
            "Wash dishes",
            "Take out trash",
            "Go shopping"
        ]
    }
"#;

struct Model {
    json_data: JsonValue,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            json_data: json::parse(MOCKUP_DATA).unwrap(),
        }
    }
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::default()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let entries: Vec<String> = self.json_data["entries"]
            .members()
            .map(ToString::to_string)
            .collect();
        html! {
            <EntryList entries={entries} />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
