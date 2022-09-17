use yew::prelude::*;
use yew_router::prelude::*;

use crate::types::{TripId};
use crate::routes::{Route};

#[derive(Clone, PartialEq)]
pub struct Trip {
    pub name: String,
    pub id: TripId,
}

pub struct TripsWidget {
    current_page: u32,
}

// Range of trips [start, end).
// They start at 0.
struct Range {
    start: u32,
    end: u32,
}

pub enum Msg {
    GoToPage(u32),
}

// This all should be from DB.
fn num_of_trips() -> u32 { 151 }
fn trips_per_page() -> u32 { 10 }
fn get_trips_in_range(range: &Range) -> Vec<Trip> {
    let mut trips = Vec::<Trip>::new();
    let end = std::cmp::min(range.end, num_of_trips());
    for i in range.start..end {
        trips.push(
            Trip { name: format!("Trip {}", i), id: TripId::from(i)}
        );
    }
    return trips;
}

fn wrap_trips(trips: &[Trip]) -> Html {
    trips.iter().map(|trip| {
        html! {
            <Link<Route> to={Route::Trip {id: trip.id}} classes={classes!("panel-block")} key={trip.id}>
                { trip.name.clone() }
            </Link<Route>>
        }
    }).collect::<Html>()
}

impl Component for TripsWidget {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_page: 1,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoToPage(dest_page) => {
                if dest_page < self.first_page() || dest_page > self.last_page() {
                    log::error!("Trying to open page {} - not in the range [{}, {}]", dest_page, self.first_page(), self.last_page());
                    false
                } else {
                    self.current_page = dest_page;
                    true
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html!{
            <div class="panel">

              <div class="panel-block">
                <p class="control has-icons-left">
                  <input class="input" type="text" placeholder="Search"/>
                  <span class="icon is-left">
                  <i class="fas fa-search" aria-hidden="true"></i>
                  </span>
                </p>
              </div>

              <div class="panel-block">
                <button class="button is-link is-outlined is-fullwidth">
                    { "+ Add new trip" }
                </button>
              </div>

              { wrap_trips(&get_trips_in_range(&self.range_for_page(self.current_page))) }

              { self.gen_pagination_controls(ctx) }

            </div>
        }
    }
}

impl TripsWidget {

    fn first_page(&self) -> u32 {
        1
    }

    fn last_page(&self) -> u32 {
        num_of_trips() / trips_per_page()
            + if num_of_trips() % trips_per_page() > 0 { 1 } else { 0 }
    }

    fn range_for_page(&self, page: u32) -> Range {
        Range {
            start: (page - 1) * trips_per_page(),
            end: page * trips_per_page(),
        }
    }

    fn gen_pagination_controls(&self, ctx: &Context<Self>) -> Html {
        let first_page = self.first_page();
        let last_page = self.last_page();

        let mut previous_classes = classes!("pagination-previous");
        let mut next_classes = classes!("pagination-next");
        let mut first_page_classes = classes!("pagination-link");
        let mut last_page_classes = classes!("pagination-link");

        if self.current_page == first_page {
            previous_classes.push("is-disabled");
            first_page_classes.push("is-disabled");
        }
        if self.current_page == last_page {
            next_classes.push("is-disabled");
            last_page_classes.push("is-disabled");
        }

        let link = ctx.link();
        let create_page_callback = |dest_page: u32| {
            link.callback(move |_: MouseEvent| Msg::GoToPage(dest_page))
        };

        html! {
          <nav class="panel-block pagination is-centered" role="navigation" aria-label="pagination">
            <button class={previous_classes} onclick={create_page_callback(self.current_page - 1)}>
              { "Previous" }
            </button>
            <button class={next_classes} onclick={create_page_callback(self.current_page + 1)}>
              { "Next page" }
            </button>
            <ul class="pagination-list">
              if self.current_page > first_page {
                <li>
                  <button class={first_page_classes} aria-label="Goto page 1" onclick={create_page_callback(first_page)}>
                    { first_page }
                  </button>
                </li>
                <li><span class="pagination-ellipsis">{ "…" }</span></li>
              }
              <li>
                <button class="pagination-link is-current" aria-label={format!("Page {}", self.current_page)} aria-current="page">
                  { self.current_page }
                </button>
              </li>
              if self.current_page < last_page {
               <li><span class="pagination-ellipsis">{ "…" }</span></li>
               <li>
                 <button class={last_page_classes} aria-label={format!("Goto page {}", last_page)} onclick={create_page_callback(last_page)}>
                   { last_page }
                 </button>
               </li>
              }
            </ul>
          </nav>
        }
    }
}
