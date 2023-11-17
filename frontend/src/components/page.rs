use yew::prelude::*;
use yew_autoprops::autoprops_component;

#[autoprops_component(Page)]
pub fn page(children: Html) -> Html {
    html! {
        <div class="container">
        { children }
        </div>
    }
}
