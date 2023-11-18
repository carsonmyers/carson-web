use yew::prelude::*;

use crate::components::Header;
use crate::components::Page;

#[function_component(Home)]
pub fn home() -> Html {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let path = location.hostname().unwrap();

    html! {
        <Page>
            <Header title={path} blink_interval={1000} />
            <div>{ "Home" }</div>
        </Page>
    }
}
