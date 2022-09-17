use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route};

#[derive(Clone, PartialEq)]
pub enum BreadcrumbElement {
    Link { text: String, destination: Route },
    Active { text: String },
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsProps {
    pub crumbs: Vec<BreadcrumbElement>,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(BreadcrumbsProps { crumbs }: &BreadcrumbsProps) -> Html {

    let links = crumbs.iter().map(|element| {
        let el_html = match element {
            BreadcrumbElement::Link{text, destination} => html!{
                <Link<Route> to={destination.clone()}> { text } </Link<Route>>
            },
            BreadcrumbElement::Active{text} => html! { <a> { text } </a> },
        };
        let mut class = classes!();
        if let BreadcrumbElement::Active{ .. } = element {
            class.push("is-active");
        }
        html! {
            <li {class}>
                { el_html }
            </li>
        }
    }).collect::<Html>();
    html! {
      <nav class="breadcrumb p-2">
        <div class="container">
          <ul>
           {links}
          </ul>
        </div>
      </nav>
    }
}
