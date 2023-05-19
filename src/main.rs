use components::me::Me;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

mod components;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Router(
                integration=HistoryIntegration::new(),
                view= move |cx, route: &ReadSignal<AppRoutes>| {
                    view! {cx,
                        div(class="app bg-indigo-300") {
                            (match route.get().as_ref() {
                                AppRoutes::Index => view! {cx,
                                    div(class="container mx-auto px-4 py-4 columns-1"){
                                        article() {
                                            Me {}
                                        }
                                        article() {
                                            Me {}
                                        }
                                        article() {
                                            Me {}
                                        }
                                    }

                                },
                                AppRoutes::About => view!{cx,
                                    article(class="flex flex-col justify-center items-center") {
                                        "This is the About page."
                                    }
                                },
                                AppRoutes::NotFound => view! {cx,
                                     "Well, you know, man, like whatever it is you are looking for ain't here. Dave's not here either."
                                }
                            })
                        }

                    }
                }
            )
        }
    });
}
