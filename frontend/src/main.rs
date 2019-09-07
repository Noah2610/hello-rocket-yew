#![recursion_limit = "256"]

extern crate failure;
extern crate json;
#[macro_use]
extern crate yew;

mod components;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use components::prelude::*;

#[derive(Default)]
struct Model {}

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
        html! {
            <TodoApp />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
