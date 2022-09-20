use yew_router::prelude::*;
use yew::prelude::*;

use crate::types::{TripId};

mod home;
mod trips;

use home::Home;
use trips::Trips;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/trips")]
    Trips,
    #[at("/trip/edit/:id")]
    TripEdit { id: TripId },
    #[at("/trip/:id")]
    Trip { id: TripId },
    #[at("/templates")]
    Templates,
    #[at("/template/edit/:template_id")]
    TemplateEdit { template_id: u64 },
    #[at("/template/:template_id")]
    Template { template_id: u64 },
    #[at("/profile")]
    Profile,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/>  },
        Route::Trips => html! { <Trips/> },
        Route::Trip { id } => html! { <h1> {format!("Trip view {} - unimplemented", id)} </h1> },
        Route::TripEdit { id } => html! { <h1> {format!("TripEdit view {} - unimplemented", id)} </h1> },
        Route::Templates => html! { <h1> {"Templates - unimplemented"} </h1> },
        Route::Template { template_id } => html! { <h1> {format!("Template view {} - unimplemented", template_id)} </h1> },
        Route::TemplateEdit { template_id } => html! { <h1> {format!("TemplateEdit view {} - unimplemented", template_id)} </h1> },
        Route::Profile => html! { <h1> {"Profile - unimplemented"} </h1> },
        Route::Login => html! { <h1> {"Login - unimplemented"} </h1> },
        Route::NotFound => html! { <h1> { "Page not found"} </h1> },
    }
}
