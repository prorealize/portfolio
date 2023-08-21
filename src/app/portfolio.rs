use yew::prelude::*;

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    html! {
      <div class="container py-16 md:py-20" id="portfolio">
      <h2
        class="text-center font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl"
      >
        {"Check out my Portfolio"}
      </h2>    
      <span
        class="pt-4 text-center font-header text-base font-normal text-black sm:text-lg lg:text-xl">
      
      </span>

      <div
        class="mx-auto grid w-full grid-cols-1 gap-8 pt-12 sm:w-3/4 md:gap-10 lg:w-full lg:grid-cols-2"
      >
        <a
          href="https://github.com/insightfulbit/mtp"
          class="mx-auto transform transition-all hover:scale-105 md:mx-0"
        >
          <img
            src="assets/img/mtp.gif"
            class="w-full shadow"
            alt="Many time pad decryptor"
          />
          <p class="mx-auto text-center">{"Many time pad decryptor (Rust)"}</p>
        </a>
        <a
          href="https://github.com/insightfulbit/password-manager"
          class="mx-auto transform transition-all hover:scale-105 md:mx-0"
        >
          <img
            src="assets/img/password-manager.jpg"
            class="w-full shadow"
            alt="Simple Password Manager"
          />
          <p class="mx-auto text-center">{"Simple Password Manager (Rust)"}</p>
        </a>        
      </div>
      <h3
        class="pt-6 text-center font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl"
      >
        {"Check other projects "} <a class="text-primary underline" href="https://www.github.com/insightfulbit">{"here"}</a>
      </h3>
    </div>
        }
}
