use yew::prelude::*;

use crate::components::*;
use crate::routes::{Route};

#[function_component(Trips)]
pub fn trips_view() -> Html {
    let crumbs = vec![
        BreadcrumbElement::Link { text: "Home".to_string(), destination: Route::Home },
        BreadcrumbElement::Active { text: "Trips".to_string() },
    ];
    html! {
        <>
            <Breadcrumbs {crumbs}/>
            <section class="section">
              <div class="container">
                <div class="columns is-centered">
                  <div class="column is-three-quarters">
                    <TripsWidget />
                  </div>
                </div>
              </div>
           </section>
        </>
    }
}
