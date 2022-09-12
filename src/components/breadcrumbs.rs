use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BreadcrumbElement {
    Link { text: String, link: String },
    Active { text: String },
}

#[derive(Properties, PartialEq)]
pub struct BreadcrumbsProps {
    pub elements: Vec<BreadcrumbElement>,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(BreadcrumbsProps { elements }: &BreadcrumbsProps) -> Html {

    let links = elements.iter().map(|element| {
        let (text, link) = match element {
            BreadcrumbElement::Link{text, link} => (text.to_string(), link.to_string()),
            BreadcrumbElement::Active{text} => (text.to_string(), "#".to_string()),
        };
        let mut class = "";
        if let BreadcrumbElement::Active{ .. } = element {
            class = "is-active";
        }
        html! {
            <li class={class}>
                <a href={link}>{ text }</a>
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
