use components::card::Card;
use components::contact::Contact;
use components::hero::Hero;
use components::nav::Nav;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

mod components;

// App state definitions
#[derive(Clone, Copy, PartialEq, Eq)]
struct MyData {
    // personal data for the Contact component
    facebook_url: &'static str,
    linkedin_url: &'static str,
    stackoverflow_url: &'static str,
    github_url: &'static str,
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct SelectState(&'static str); // state for the select/option elements for small screens
impl SelectState {
    pub fn path(&self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct TabStateData {
    // tailwindcss class data for the Nav component's tabs
    selected_anchor_classes: &'static str,
    unselected_anchor_classes: &'static str,
    selected_span_classes: &'static str,
    unselected_span_classes: &'static str,
}
impl TabStateData {
    fn new() -> Self {
        Self {
            selected_anchor_classes: "bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
            unselected_anchor_classes: "bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
            selected_span_classes: "bg-pink-500 absolute inset-x-0 bottom-0 h-0.5",
            unselected_span_classes: "bg-transparent absolute inset-x-0 bottom-0 h-0.5",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct CurrentTabState {
    // state to hold the tailwindcss classes for the currently selected and unselected tabs
    project_anchor_classes: &'static str,
    project_span_classes: &'static str,
    about_anchor_classes: &'static str,
    about_span_classes: &'static str,
    contact_anchor_classes: &'static str,
    contact_span_classes: &'static str,
}
impl CurrentTabState {
    fn new() -> Self {
        Self {
            project_anchor_classes: "",
            project_span_classes: "",
            about_anchor_classes: "",
            about_span_classes: "",
            contact_anchor_classes: "",
            contact_span_classes: "",
        }
    }
    fn select_project(&self, tab_state_data: &TabStateData) -> Self {
        // classes to style the currently selected Projects tab and unselected About/Contact tabs
        Self {
            project_anchor_classes: tab_state_data.selected_anchor_classes,
            project_span_classes: tab_state_data.selected_span_classes,
            about_anchor_classes: tab_state_data.unselected_anchor_classes,
            about_span_classes: tab_state_data.unselected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_about(&self, tab_state_data: &TabStateData) -> Self {
        // classes to style the currently selected About tab and unselected Projects/Contact tabs
        Self {
            project_anchor_classes: tab_state_data.unselected_anchor_classes,
            project_span_classes: tab_state_data.unselected_span_classes,
            about_anchor_classes: tab_state_data.selected_anchor_classes,
            about_span_classes: tab_state_data.selected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_contact(&self, tab_state_data: &TabStateData) -> Self {
        // classes to style the currently selected Contact tab and unselected Projects/About tabs
        Self {
            project_anchor_classes: tab_state_data.unselected_anchor_classes,
            project_span_classes: tab_state_data.unselected_span_classes,
            about_anchor_classes: tab_state_data.unselected_anchor_classes,
            about_span_classes: tab_state_data.unselected_span_classes,
            contact_anchor_classes: tab_state_data.selected_anchor_classes,
            contact_span_classes: tab_state_data.selected_span_classes,
        }
    }
}

// App routes
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
        // Contact state setup
        let my_data = MyData {
            facebook_url: "https://www.facebook.com/BHBoruff/",
            linkedin_url: "https://www.linkedin.com/in/benjaminboruff/",
            stackoverflow_url: "https://stackoverflow.com/users/6026248/benjamin-h-boruff",
            github_url: "https://github.com/benjaminboruff",
        };
        let my_data_ref = create_ref(cx, my_data);
        provide_context_ref(cx, my_data_ref);
        // Nav state setup
        let select_state = create_signal(cx, SelectState("/"));
        provide_context_ref(cx, select_state);
        let tab_state_data = TabStateData::new();
        let tab_state_data_ref = create_ref(cx, tab_state_data);
        provide_context_ref(cx, tab_state_data_ref);
        let current_tab_state =
            create_signal(cx, CurrentTabState::new().select_project(&tab_state_data));
        provide_context_ref(cx, current_tab_state);

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
