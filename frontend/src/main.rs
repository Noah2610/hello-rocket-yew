extern crate yew;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <h1>{ "Hello World!" }</h1>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
