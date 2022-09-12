use yew::prelude::*;

mod components;
use components::*;

#[function_component(App)]
fn app() -> Html {
    let crumbs = vec![
        BreadcrumbElement::Link{ text: "Home".to_string(), link: "index.html".to_string()},
        BreadcrumbElement::Active{ text: "HERE".to_string() },
    ];
    html! {
        <>
            <HeaderBar/>
            <Breadcrumbs elements={crumbs} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
