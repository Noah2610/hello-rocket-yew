pub mod prelude {
    pub use super::todo_app::prelude::*;
}

mod component_prelude {
    pub use super::helpers::*;
    pub use super::prelude::*;
    pub use failure::Error;
    pub use yew::format;
    pub use yew::services::fetch::{
        FetchService,
        FetchTask,
        Request,
        Response,
        StatusCode,
    };
    pub use yew::{
        html,
        Component,
        ComponentLink,
        Html,
        Renderable,
        ShouldRender,
    };
}

mod todo_app;

mod helpers {
    pub mod console {
        use yew::services::console::ConsoleService;

        pub fn log<S: ToString>(msg: S) {
            ConsoleService::new().log(msg.to_string().as_str());
        }
        pub fn warn<S: ToString>(msg: S) {
            ConsoleService::new().warn(msg.to_string().as_str());
        }
        pub fn error<S: ToString>(msg: S) {
            ConsoleService::new().error(msg.to_string().as_str());
        }
        pub fn debug<S: ToString>(msg: S) {
            ConsoleService::new().debug(msg.to_string().as_str());
        }
    }
}
