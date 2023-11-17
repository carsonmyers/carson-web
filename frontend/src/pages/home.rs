use yew::prelude::*;

use crate::components::Header;
use crate::components::Page;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Page>
            <Header title="Header Title" blink_interval={1000} />
            <div>{ "Home" }</div>
        </Page>
    }
}
