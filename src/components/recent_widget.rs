use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum RecentWidgetElementType {
    Trip,
    Template,
}

#[derive(Clone, PartialEq)]
pub struct RecentWidgetElement {
    pub name: String,
    // Probably a relevant id in future
    pub el_type: RecentWidgetElementType,
}

#[derive(Properties, PartialEq)]
pub struct RecentWidgetProps {
    pub title: String,
    pub elements: Vec<RecentWidgetElement>,
}

#[function_component(RecentWidget)]
pub fn recent_widget(RecentWidgetProps { title, elements }: &RecentWidgetProps) -> Html {

    let html_els = elements.iter().map(|element| {
        let mut tag_name = "a";
        let mut classes = "panel-block".to_string();
        let mut link = "".to_string();
        if element.el_type == RecentWidgetElementType::Template {
            tag_name = "div";
            classes.push_str(" is-justify-content-space-between");
            link.push_str("href=\"link.html\"");
        }
        html! {
            <@{tag_name} class={classes} {link}>
                <div> {element.name.to_string()} </div>
                if element.el_type == RecentWidgetElementType::Template {
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
