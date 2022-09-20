use yew::prelude::*;
use yew_router::prelude::*;

use crate::types::{UserAuthStatus};
use crate::routes::{Route};

#[function_component(HeaderBar)]
pub fn header_bar() -> Html {
    let user_auth = if let Some(user_auth) = use_context::<UserAuthStatus>() {
        user_auth
    } else {
        UserAuthStatus::new_empty()
    };

    let profile_button = if let Some(user_info) = user_auth.user_info {
        html! {
          <Link<Route> to={Route::Profile} classes={classes!("bd-tw-button", "button")}>
            <span class="icon">
              <i class="fas fa-user"></i>
            </span>
            <span>
              { user_info.display_name }
            </span>
          </Link<Route>>
        }
    } else {
        html! {
          <Link<Route> to={Route::Login} classes={classes!("bd-tw-button", "button")}>
            <span class="icon">
              <i class="fas fa-sign-in"></i>
            </span>
            <span>
              { "Log in" }
            </span>
          </Link<Route>>
        }
    };

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

        <div class="navbar-end">
          <div class="navbar-item">
            <div class="field is-grouped">
              <p class="control">
                { profile_button }
              </p>
            </div>
          </div>
        </div>

      </nav>
    }
}
