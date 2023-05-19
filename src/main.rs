use components::card::Card;
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
                view=|cx, route: &ReadSignal<AppRoutes>| {
                    let count = create_signal(cx, vec![1,2,3,4]);
                    view! { cx,
                        div(class="app min-h-screen bg-sky-400") {
                            div(class="content") {
                                div(class="container mx-auto p-4"){
                                    div(class="flex") {
                                        (match route.get().as_ref() {
                                            AppRoutes::Index => view! {cx,
                                                div(class="flex-col mx-auto"){
                                                    Keyed(
                                                        iterable=count,
                                                        view=|cx, x| view! { cx,
                                                            Card(item=x){}
                                                        },
                                                        key=|x| *x,
                                                    )
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
                        }

                    }
                }
            )
        }
    });
}
