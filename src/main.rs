use components::button::Button;
use components::count::Count;
use components::square::Square;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

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
        let state = create_signal(cx, 0i32);
        let update = create_ref(cx, |action| {
            if action == 0 {
                state.set(0);
            } else {
                state.set(*state.get() + action);
            }
        });
        view! { cx,
            Router(
                integration=HistoryIntegration::new(),
                view= move |cx, route: &ReadSignal<AppRoutes>| {
                    view! {cx,
                        div(class="app") {
                            (match route.get().as_ref() {
                                AppRoutes::Index => view! {cx,
                                    article(class="flex flex-col justify-center items-center") {
                                        Count(value=state)
                                        header() {
                                            Button(updater=update, action=-1)
                                            Button(updater=update, action=0)
                                            Button(updater=update, action=1)
                                        }
                                        Square(value=state)
                                    }
                                },
                                AppRoutes::About => view!{cx,
                                    "This is the About page."
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
