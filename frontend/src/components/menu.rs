use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Menu)]
pub fn menu() -> Html {
    let current_route = use_route::<Route>().unwrap();
    log!("current route:", format!("{:?}", current_route));

    let links = [(Route::Home, "home"), (Route::Colors, "colors")]
        .into_iter()
        .map(|(route, text)| {
            log!(
                "check",
                format!(
                    "{:?} = {:?}: {}",
                    &route,
                    current_route,
                    &route.eq(&current_route)
                )
            );
            if route.eq(&current_route) {
                html! {
                    <a class="current_page">
                        { format!("[ {} ]", text)}
                    </a>
                }
            } else {
                html! {
                    <Link<Route> to={route}>{ format!("[ {} ]", text) }</Link<Route>>
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
