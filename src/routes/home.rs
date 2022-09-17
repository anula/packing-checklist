use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::*;
use crate::routes::{Route};

#[function_component(Home)]
pub fn home_view() -> Html {
    let trips = vec![
        RecentWidgetElement{
            name: "Trip 1".to_string(),
            el_type: RecentWidgetElementType::Trip,
        },
        RecentWidgetElement{
            name: "Trip 2".to_string(),
            el_type: RecentWidgetElementType::Trip,
        },
        RecentWidgetElement{
            name: "Trip 3".to_string(),
            el_type: RecentWidgetElementType::Trip,
        },
    ];
    let templates = vec![
        RecentWidgetElement{
            name: "Template 1".to_string(),
            el_type: RecentWidgetElementType::Template,
        },
        RecentWidgetElement{
            name: "Template 2".to_string(),
            el_type: RecentWidgetElementType::Template,
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
