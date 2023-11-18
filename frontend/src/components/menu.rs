use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Menu)]
pub fn menu() -> Html {
    let current_route = use_route::<Route>().unwrap();

    let links = [(Route::Home, "home"), (Route::Colors, "colors")]
        .into_iter()
        .map(|(route, text)| {
            if route.eq(&current_route) {
                html! {
                    <a class="current_page">
                        { format!("[ {} ]", text) }
                    </a>
                }
            } else {
                html! {
                    <Link<Route> to={route}>{ text }</Link<Route>>
                }
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div class="menu">
            {links}
        </div>
    }
}
