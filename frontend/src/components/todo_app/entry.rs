use super::component_prelude::*;

#[derive(Properties, Debug, Deserialize)]
pub struct Props {
    pub name:      String,
    pub completed: bool,
}

pub struct Entry {
    name:      String,
    completed: bool,
}

impl Component for Entry {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name:      props.name,
            completed: props.completed,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.name != props.name || self.completed != props.completed {
            self.name = props.name;
            self.completed = props.completed;
            true
        } else {
            false
        }
    }
}

impl Renderable<Self> for Entry {
    fn view(&self) -> Html<Self> {
        html! {
            if self.completed {
                html! {
                    <span style="text-decoration: line-through;">
                        { &self.name }
                    </span>
                }
            } else {
                html! {
                    &self.name
                }
            }
        }
    }
}
