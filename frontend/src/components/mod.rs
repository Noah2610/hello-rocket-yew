pub mod prelude {
    pub use super::entry_list::EntryList;
}

mod component_prelude {
    pub use yew::{
        html,
        Component,
        ComponentLink,
        Html,
        Renderable,
        ShouldRender,
    };
}

mod entry_list;
