use super::component_prelude::*;

#[derive(Properties)]
pub struct Props {
    pub entries: Vec<String>,
}

pub struct EntryList {
    entries: Vec<String>,
}

impl Component for EntryList {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        EntryList {
            entries: props.entries,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.entries = props.entries;
        true
    }
}

impl Renderable<EntryList> for EntryList {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "TODOS" }</h1>
                {
                    if self.entries.is_empty() {
                        html! {
                            <strong>{ "No todos!" }</strong>
                        }
                    } else {
                        html! {
                            <ul>{
                                for self.entries
                                    .iter()
                                        .map(|entry| html! {
                                            <li>{
                                                entry
                                            }</li>
                                        })
                            }</ul>
                        }
                    }
                }
            </div>
        }
    }
}
