use yew::prelude::*;
use yew_icons::{Icon, IconId};

/// About page
#[function_component(About)]
pub fn about() -> Html {
    html! {
  <div class="bg-grey-50" id="about">
    <div class="container items-center py-16 md:py-20">
    <div class="w-full text-center sm:w-3/4 lg:w-4/5 mx-auto">
      <h2
        class="font-header text-4xl font-semibold uppercase text-primary sm:text-5xl lg:text-6xl"
      >
        {"Who am I?"}
      </h2>
      <h4
        class="pt-6 font-header text-xl font-medium text-black sm:text-2xl lg:text-3xl"
      >
        {"I'm Renato, a Lead Backend Software Developer"}
      </h4>
      <p class="pt-6 font-body leading-relaxed text-grey-20">
        {"Living by the kaizen philosophy of continuous incremental improvements, I've embraced the challenges and thrills that come with being a self-taught developer. It's not just a career for me but an adventure filled with fun, learning, and growth. With over 3 years of experience, I've cultivated a passion for Rust, Machine Learning, and Cybersecurity, with a specific emphasis on encryption. The joy I find in coding has led me to explore diverse projects, always up for a challenge."}
      </p>
      <p class="pt-6 font-body leading-relaxed text-grey-20">
        {"This website is a perfect example of this. I built using the Yew framework to demonstrate my Rust skills. It was not just about proving my expertise but indulging in the sheer pleasure of crafting something from scratch, experimenting with new techniques, and finding creative solutions. From writing robust Rust code to exploring the frontiers of machine learning and fortifying digital landscapes, my journey has been marked by curiosity, resilience, and a contagious enthusiasm for all things tech."}
      </p>
      <p class="pt-6 font-body leading-relaxed text-grey-20">
        {"Whether it's working solo on a project or collaborating in a team environment, the joy I find in coding propels me to innovate, inspire, and excel, making me a top producer in every company I've worked for."}
      </p>
      <div
        class="flex justify-center pt-6 sm:flex-row lg:justify-start"
      >
        <div class="flex items-center justify-center sm:justify-start">
          <p class="font-body text-lg font-semibold uppercase text-grey-20">
            {"Connect with me"}
          </p>
          <div class="hidden sm:block">
            <Icon class="text-2xl text-primary" icon_id={IconId::FontAwesomeSolidChevronRight}/>
          </div>
        </div>
        <div
          class="flex items-center justify-center pt-5 pl-2 sm:justify-start sm:pt-0"
        >
        <a href="https://www.linkendin.com/in/reaal" class="pl-4"><Icon class="text-2xl text-primary hover:text-yellow" icon_id={IconId::BootstrapLinkedin}/></a>
        <a href="https://medium.com/@insightfulbit" class="pl-4"><Icon class="text-2xl text-primary hover:text-yellow" icon_id={IconId::BootstrapMedium}/></a>
        <a href="https://www.github.com/insightfulbit" class="pl-4"><Icon class="text-2xl text-primary hover:text-yellow" icon_id={IconId::BootstrapGithub}/></a>
        <a href="#contact" class="pl-4"><Icon class="text-2xl text-primary hover:text-yellow" icon_id={IconId::LucideMail}/></a>          
        </div>
      </div>
    </div>    
  </div>
  </div>
 }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use wasm_bindgen_test::*;
    use yew::platform::time::sleep;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;

    #[wasm_bindgen_test]
    async fn about_page_has_an_app_link() {
        yew::Renderer::<About>::with_root(
            gloo_utils::document().get_element_by_id("output").unwrap(),
        )
        .render();

        sleep(Duration::ZERO).await;

        let app_links = gloo_utils::document().get_elements_by_tag_name("a");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
