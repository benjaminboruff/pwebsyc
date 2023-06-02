use components::about::About;
use components::contact::Contact;
use components::hero::Hero;
use components::nav::Nav;
use components::projects::Projects;
// use log::info;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{HistoryIntegration, Route, Router};

mod components;

// The tab selection state -- the route determines the styling, and tracking which option in the
// select element has been chosen
#[derive(Clone, Copy, PartialEq, Eq)]
struct TabRoute(&'static str); // for syncing the router with the nav bar tabs
impl TabRoute {
    fn path(self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SelectionValue(&'static str); // for syncing the selected option with the nav bar selection
impl SelectionValue {
    fn value(self) -> &'static str {
        self.0
    }
}

// app routes
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

// github api to retrieve all my repo data
const API_BASE_URL: &str = "https://api.github.com/users/benjaminboruff/repos";

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repository {
    name: String,
    html_url: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(transparent)]
struct Repositories {
    repositories: Vec<Repository>,
}

// function that fetches my repo json data on each page load
async fn fetch_all_projects<G: Html>() -> Result<Repositories, reqwasm::Error> {
    let url = format!("{}{}", API_BASE_URL, "?per_page=100");
    let resp = Request::get(&url).send().await?;

    let resp_str = resp.text().await?;
    let repositories: Repositories = serde_json::from_str(&resp_str.as_str())?;
    Ok(repositories)
}

#[component]
async fn App<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    // fetch github data for projects component
    let github = fetch_all_projects::<G>().await.unwrap_or_default();
    let mut repos: Vec<Repository> = github.repositories;
    const TOP_SIX_REPOS: &[&str] = &[
        "minigrep",
        "blasteroids",
        "calculator3",
        "calculator2",
        "pomodoro",
        "pwebsyc",
    ];
    // only keep the repos that are included in the array of top six repos
    repos.retain(|repo| TOP_SIX_REPOS.contains(&repo.name.as_str()));
    let github_repos = create_signal(cx, repos);
    provide_context_ref(cx, github_repos);

    // router path for use in Nav{} to sync tabs with browser route
    let router_path = create_signal(cx, TabRoute("/"));
    provide_context_ref(cx, router_path);
    // selection value for use in Nav{} to sync selection with selected option
    let selection_value = create_signal(cx, SelectionValue("Projects"));
    provide_context_ref(cx, selection_value);

    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="app min-h-screen bg-sky-400") {
                        div(class="text-gray-900 font-sans") {
                            Hero{}
                            div {
                                (match route.get().as_ref() {
                                    AppRoutes::Index => {
                                        router_path.set(TabRoute("/"));
                                        // info!("{}", router_path.get().path());
                                        selection_value.set(SelectionValue("Projects"));
                                        // info!("{}", selection_value.get().value());
                                         view! {cx, // Projects
                                            Nav(route=router_path.get().path(), select_value=selection_value.get().value()){}
                                            div(class="container mx-auto p-4") {
                                                Suspense(fallback=view! { cx, div(class="flex flex-col justify-center items-center text-lg leading-8 text-gray-700") { "Loading..." } }) {
                                                    Projects{}
                                                }
                                            }
                                        }
                                    }
                                    ,
                                    AppRoutes::About => {
                                        router_path.set(TabRoute("/about"));
                                        // info!("{}", router_path.get().path());
                                        selection_value.set(SelectionValue("About"));
                                        // info!("{}", selection_value.get().value());
                                        view! {cx,
                                                Nav(route=router_path.get().path(), select_value=selection_value.get().value()){}
                                                About{}
                                            }
                                    },
                                    AppRoutes::Contact =>{
                                        router_path.set(TabRoute("/contact"));
                                        // info!("{}", router_path.get().path());
                                        selection_value.set(SelectionValue("Contact"));
                                        // info!("{}", selection_value.get().value());
                                        view!{cx,
                                            Nav(route=router_path.get().path(), select_value=selection_value.get().value()){}
                                            Contact{}
                                        }
                                     }
                                    ,
                                    AppRoutes::NotFound => view! {cx,
                                        div(class="flex flex-col justify-center items-center") {
                                            div(class="m-4") {
                                                p (class="text-lg leading-8 text-gray-700"){"Well, you know, man, like, whatever it is you are looking for ain't here."}
                                                p (class="text-lg leading-8 text-gray-700"){
                                                    a(class="underline", href="https://www.youtube.com/watch?v=dX8lZduI8ac") {"Dave's not here"}
                                                    ", either."
                                                }

                                            }
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
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
