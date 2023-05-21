use components::card::Card;
use components::contact::Contact;
use components::hero::Hero;
use components::nav::Nav;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

mod components;

#[derive(Clone, Copy, PartialEq, Eq)]
struct MyData {
    facebook_url: &'static str,
    linkedin_url: &'static str,
    stackoverflow_url: &'static str,
    github_url: &'static str,
}

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[to("/contact")]
    Contact,
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| {
        let my_data = MyData {
            facebook_url: "https://www.facebook.com/BHBoruff/",
            linkedin_url: "https://www.linkedin.com/in/benjaminboruff/",
            stackoverflow_url: "https://stackoverflow.com/users/6026248/benjamin-h-boruff",
            github_url: "https://github.com/benjaminboruff",
        };

        let my_data_ref = create_ref(cx, my_data);
        provide_context_ref(cx, my_data_ref);

        view! { cx,
            Router(
                integration=HistoryIntegration::new(),
                view=|cx, route: &ReadSignal<AppRoutes>| {
                    let count = create_signal(cx, vec![1,2,3,4]); // placeholder for project data
                    view! { cx,
                        div(class="app min-h-screen bg-sky-400") {
                            div(class="text-gray-900 font-sans") {
                                Hero{}
                                Nav{}
                                div(class="container p-4 mx-auto"){
                                    (match route.get().as_ref() {
                                        AppRoutes::Index => view! {cx,
                                                Keyed(
                                                    iterable=count,
                                                    view=|cx, x| view! { cx,
                                                        Card(item=x){}
                                                    },
                                                    key=|x| *x,
                                                )
                                        },
                                        AppRoutes::About => view!{cx,
                                            div(class="flex flex-col justify-center items-center") {
                                                p(class="text-lg leading-8 text-gray-700") {"This is the About page."}
                                            }
                                        },
                                        AppRoutes::Contact => view!{cx,
                                            Contact{}
                                        },
                                        AppRoutes::NotFound => view! {cx,
                                            div(class="flex flex-col justify-center items-center") {
                                                p (class="text-lg leading-8 text-gray-700"){"Well, you know, man, like whatever it is you are looking for ain't here."}
                                                p (class="text-lg leading-8 text-gray-700"){"Dave's not here, either."}
                                            }
                                        }
                                    })
                                }
                            }
                        }

                    }
                }
            )
        }
    });
}
