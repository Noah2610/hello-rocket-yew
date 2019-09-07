use super::component_prelude::*;
use super::entry::Props as EntryProps;

#[derive(Properties)]
pub struct Props {
    pub entries: Vec<EntryProps>,
    #[props(required)]
    pub toggle_entry_completed: Callback<usize>,
}

pub enum Msg {
    ToggleCompleted(usize),
    None,
}

pub struct EntryList {
    entries:                Vec<EntryProps>,
    toggle_entry_completed: Callback<usize>,
}

impl Component for EntryList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        EntryList {
            entries:                props.entries,
            toggle_entry_completed: props.toggle_entry_completed,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleCompleted(id) => {
                self.toggle_entry_completed.emit(id);
            }
            Msg::None => (),
        }
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
                                        <div onclick=|_| Msg::ToggleCompleted(0)>
                                            <li>
                                                <Entry
                                                 id = entry.id
                                                 name = entry.name.clone()
                                                 completed = entry.completed />
                                            </li>
                                        </div>
                                    })
                            }</ul>
                        }
                    }
                }
            </div>
        }
    }
}
