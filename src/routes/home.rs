use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::*;
use crate::routes::{Route};

#[function_component(Home)]
pub fn home_view() -> Html {
    let trips = vec![
        recent_widget::Element {
            name: "Trip 1".to_string(),
            el_type: recent_widget::ElementType::Trip,
        },
        recent_widget::Element {
            name: "Trip 2".to_string(),
            el_type: recent_widget::ElementType::Trip,
        },
        recent_widget::Element {
            name: "Trip 3".to_string(),
            el_type: recent_widget::ElementType::Trip,
        },
    ];
    let templates = vec![
        recent_widget::Element {
            name: "Template 1".to_string(),
            el_type: recent_widget::ElementType::Template,
        },
        recent_widget::Element {
            name: "Template 2".to_string(),
            el_type: recent_widget::ElementType::Template,
        },
    ];
    html! {
        <section class={ "section" }>
            <div class="container">
                <div class="columns is-centered">
                    <div class="column is-two-thirds">
                        <RecentWidget title={"Recent trips"} elements={trips} />
                        <Link<Route> to={Route::Trips}>{">> Go to Trips"}</Link<Route>>
                    </div>
                    <div class="column is-one-third">
                        <RecentWidget title={"Recent templates"} elements={templates} />
                        <Link<Route> to={Route::Templates}>{">> Go to Templates"}</Link<Route>>
                    </div>
                </div>
            </div>
        </section>
    }
}
