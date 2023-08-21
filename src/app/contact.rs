use yew::prelude::*;
use yew_icons::{Icon, IconId};

/// Home page
#[function_component(Contact)]
pub fn contact() -> Html {

    html! {
        <div class="container py-16 md:py-20" id="contact">
  <h2
    class="text-center font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl"
  >
    {"Contact me"}
  </h2>
  <h4
    class="pt-6 text-center font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl"
  >
    {"Have Any Questions?"}
  </h4>
  <form class="mx-auto w-full pt-10 sm:w-3/4" action="https://formspree.io/f/mwkdolgl" method="POST">
    <div class="flex flex-col md:flex-row">
      <input
        class="mr-3 w-full rounded border-grey-50 px-4 py-3 font-body text-black md:w-1/2 lg:mr-5"
        placeholder="Name"
        name="name"
        type="text"
        id="name"
      />
      <input
        class="mt-6 w-full rounded border-grey-50 px-4 py-3 font-body text-black md:mt-0 md:ml-3 md:w-1/2 lg:ml-5"
        placeholder="Email"
        name="email"
        type="email"
        id="email"
      />
    </div>
    <textarea
      class="mt-6 w-full rounded border-grey-50 px-4 py-3 font-body text-black md:mt-8"
      placeholder="Message"
      name="message"
      type="message"
      id="message"
      cols="30"
      rows="10"
    ></textarea>
    <button
      class="mt-6 flex items-center justify-center rounded bg-primary px-8 py-3 font-header text-lg font-bold uppercase text-white hover:bg-grey-20"
    >
      {"Send"}
      <Icon class="relative -right-2 text-3xl" icon_id={IconId::FontAwesomeSolidChevronRight}/>
    </button>
  </form>
</div>
        }}