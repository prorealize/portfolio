use yew::prelude::*;
use yew_icons::{Icon, IconId};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {

    html! {
            <div>
                <div class="relative bg-cover bg-center bg-no-repeat py-8" style="background-image: url('assets/img/bg-hero.jpg')">
                    <div class="absolute inset-0 z-20 bg-gradient-to-r from-hero-gradient-from to-hero-gradient-to bg-cover bg-center bg-no-repeat"></div>
                    <div class="container relative z-30 pt-20 pb-12 sm:pt-56 sm:pb-48 lg:pt-64 lg:pb-48">
                        <div class="flex flex-col items-center justify-center lg:flex-row">
                            <div class="rounded-full border-8 border-primary shadow-xl">
                                <img class="h-48 rounded-full sm:h-56" src="assets/img/profile.jpg" alt="author"/>
                            </div>
                            <div class="pt-8 sm:pt-10 lg:pl-8 lg:pt-0">
                                <h1 class="text-center font-header text-4xl text-white sm:text-left sm:text-5xl md:text-6xl">
                                    { "Hello, I'm Renato!" }
                                </h1>
                                <div class="flex flex-col justify-center pt-3 sm:flex-row sm:pt-5 lg:justify-start">
                                    <div class="flex items-center justify-center pl-0 sm:justify-start md:pl-1">
                                        <p class="font-body text-lg uppercase text-white">{ "Let's connect" }</p>
                                        <div class="hidden sm:block">
                                            <Icon class="text-3xl text-yellow" icon_id={IconId::FontAwesomeSolidChevronRight}/>
                                        </div>
                                    </div>
                                    // Social Media Links
                                    <div class="flex items-center justify-center pt-5 pl-2 sm:justify-start sm:pt-0">
                                        <a href="https://www.linkendin.com/in/reaal" class="pl-4"><Icon class="text-2xl text-white hover:text-yellow" icon_id={IconId::BootstrapLinkedin}/></a>
                                        <a href="https://medium.com/@insightfulbit" class="pl-4"><Icon class="text-2xl text-white hover:text-yellow" icon_id={IconId::BootstrapMedium}/></a>
                                        <a href="https://www.github.com/insightfulbit" class="pl-4"><Icon class="text-2xl text-white hover:text-yellow" icon_id={IconId::BootstrapGithub}/></a>
                                        <a href="#contact" class="pl-4"><Icon class="text-2xl text-white hover:text-yellow" icon_id={IconId::LucideMail}/></a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}
