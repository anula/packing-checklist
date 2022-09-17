use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod routes;

use crate::components::*;
use crate::routes::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <HeaderBar/>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
