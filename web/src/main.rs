use app::App;

mod component;
mod layout;
mod page;
mod app;
mod router;
mod state;
mod r#const;
mod util;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<App>::new().render();
}
