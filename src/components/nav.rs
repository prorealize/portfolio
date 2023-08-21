use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {

  let sections = vec![
    ("#about", "About"),
    ("#portfolio", "Portfolio"),
    ("#contact", "Contact"),
];

let links: Vec<Html> = sections
        .iter()
        .map(|(section_link, link_text)| render_section_link(section_link, link_text))
        .collect();

    html! {
      <div>
      <div class="w-full z-50 top-0 py-3 sm:py-5  absolute">
      <div class="container flex items-center justify-between">
        <div>
          <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >
              <img class="w-12 lg:w-15" src="assets/img/logo.png" alt="InsightfulBit Logo" />
          </Link<AppRoute>>
        </div>
        <div class="hidden lg:block">
          <ul class="flex items-center">
          { for links.into_iter() }
          </ul>
        </div>
      </div>
    </div>

          </div>
        }
}

fn render_section_link(section: &str, link_text: &str) -> Html {
  let section: String = section.to_owned();
  let link_text = link_text.to_owned();
    html! {
        <li class="group pl-6">
          <a 
          href={ section } class="cursor-pointer pt-0.5 font-header font-semibold uppercase text-white">{ link_text }</a>
          // Underline Bar
          <span class="block h-0.5 w-full bg-transparent group-hover:bg-yellow"></span>
        </li>
    }
}