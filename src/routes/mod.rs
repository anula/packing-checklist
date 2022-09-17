use yew_router::prelude::*;
use yew::prelude::*;

mod home;

pub use home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/trips")]
    Trips,
    #[at("/trip/edit/:trip_id")]
    TripEdit { trip_id: u64 },
    #[at("/trip/:trip_id")]
    Trip { trip_id: u64 },
    #[at("/templates")]
    Templates,
    #[at("/template/edit/:template_id")]
    TemplateEdit { template_id: u64 },
    #[at("/template/:template_id")]
    Template { template_id: u64 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/>  },
        Route::Trips => html! { <h1> {"Trips - unimplemented"} </h1> },
        Route::Trip { trip_id } => html! { <h1> {format!("Trip view {} - unimplemented", trip_id)} </h1> },
        Route::TripEdit { trip_id } => html! { <h1> {format!("TripEdit view {} - unimplemented", trip_id)} </h1> },
        Route::Templates => html! { <h1> {"Templates - unimplemented"} </h1> },
        Route::Template { template_id } => html! { <h1> {format!("Template view {} - unimplemented", template_id)} </h1> },
        Route::TemplateEdit { template_id } => html! { <h1> {format!("TemplateEdit view {} - unimplemented", template_id)} </h1> },
        Route::NotFound => html! { <h1> { "Page not found"} </h1> },
    }
}
