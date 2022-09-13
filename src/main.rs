use yew::prelude::*;

mod components;
use components::*;

#[function_component(App)]
fn app() -> Html {
    let crumbs = vec![
        BreadcrumbElement::Link{ text: "Home".to_string(), link: "index.html".to_string()},
        BreadcrumbElement::Active{ text: "HERE".to_string() },
    ];
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
        <>
            <HeaderBar/>
            <Breadcrumbs elements={crumbs} />
            <section class={ "section" }>
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-two-thirds">
                            <RecentWidget title={"Recent trips"} elements={trips} />
                        </div>
                        <div class="column is-one-third">
                        <RecentWidget title={"Recent templates"} elements={templates} />
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
