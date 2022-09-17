use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ElementType {
    Trip,
    Template,
}

#[derive(Clone, PartialEq)]
pub struct Element {
    pub name: String,
    // Probably a relevant id in future
    pub el_type: ElementType,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub elements: Vec<Element>,
}

pub struct RecentWidget;

impl Component for RecentWidget {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RecentWidget
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { title, elements } = ctx.props();
        let html_els = elements.iter().map(|element| {
            let mut tag_name = "a";
            let mut class = classes!("panel-block");
            let mut href = Some("link.html");
            if element.el_type == ElementType::Template {
                tag_name = "div";
                class.push("is-justify-content-space-between");
                href = None; 
            }
            html! {
                <@{tag_name} {class} {href} >
                    <div> {element.name.to_string()} </div>
                    if element.el_type == ElementType::Template {
                      <div>
                        <button class="button is-link is-light is-small">
                            { "Create trip" }
                        </button>
                      </div>
                    }
                </@>
            }
        }).collect::<Html>();
        html! {
          <div class="panel">

            <p class="panel-heading">{title}</p>

            { html_els }

          </div>
        }
    }
}
