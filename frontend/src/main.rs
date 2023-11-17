mod app;
mod common;
mod components;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
