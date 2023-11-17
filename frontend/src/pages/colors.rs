use yew::prelude::*;

use crate::components::ColorTable;
use crate::components::Header;
use crate::components::Page;

#[function_component(Colors)]
pub fn colors() -> Html {
    html! {
        <Page>
            <Header title="Colors" blink_interval={1000} />
            <ColorTable />
        </Page>
    }
}
