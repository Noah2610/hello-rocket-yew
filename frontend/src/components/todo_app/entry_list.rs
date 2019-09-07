use super::component_prelude::*;
use super::entry::Props as EntryProps;

#[derive(Properties)]
pub struct Props {
    pub entries: Vec<EntryProps>,
}

pub struct EntryList {
    entries: Vec<EntryProps>,
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

impl Renderable<Self> for EntryList {
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
                                        <li key=entry.id>
                                            <Entry name=entry.name.clone()
                                             completed=entry.completed />
                                        </li>
                                    })
                            }</ul>
                        }
                    }
                }
            </div>
        }
    }
}
