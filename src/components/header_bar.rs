use yew::prelude::*;

#[function_component(HeaderBar)]
pub fn header_bar() -> Html {
    html! {
      <nav class="navbar has-shadow is-primary" role="navigation" aria-label="main navigation">
        <div class="navbar-brand px-2">
          <a class="navbar-item has-text-weight-bold is-size-3 is-family-sans-serif">
            <span class="icon-text">
              <span class="icon">
                <i class="fas fa-briefcase"></i>
              </span>
              <span class="px-2">{ "Packing checklist" }</span>
            </span>
          </a>
        </div>
      </nav>
    }
}
