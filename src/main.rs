use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod config;
mod firebase;
mod routes;
mod types;

use crate::components::*;
use crate::routes::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    // TODO: replace ConfigProvider with FirebaseAuthProvider(?)
    html! {
        <ConfigProvider>
          <UserContextProvider>
            <BrowserRouter>
              <HeaderBar/>
              <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
          </UserContextProvider>
        </ConfigProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
