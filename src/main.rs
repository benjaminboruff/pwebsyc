use components::about::About;
use components::contact::Contact;
use components::hero::Hero;
use components::nav::Nav;
use components::projects::Projects;
use log::info;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{HistoryIntegration, Route, Router};

mod components;

// app data structures
#[derive(Clone, Copy, PartialEq, Eq)]
struct SiteData {
    // generic site data that could be used anywhere
    facebook_url: &'static str,
    linkedin_url: &'static str,
    stackoverflow_url: &'static str,
    github_url: &'static str,
}
impl SiteData {
    fn new() -> Self {
        Self {
            facebook_url: "http://facebook.com",
            linkedin_url: "http://linkedin.com",
            stackoverflow_url: "http://stackoverflow.com",
            github_url: "http://github.com",
        }
    }
    fn builder(
        facebook_url: &'static str,
        linkedin_url: &'static str,
        stackoverflow_url: &'static str,
        github_url: &'static str,
    ) -> Self {
        Self {
            facebook_url,
            linkedin_url,
            stackoverflow_url,
            github_url,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Page {
    name: &'static str,
    route: &'static str,
    selected: bool,
}
impl Page {
    fn builder(name: &'static str, route: &'static str, selected: bool) -> Self {
        Self {
            name,
            route,
            selected,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct TabRoute(&'static str); // for syncing the router with the nav bar tabs
#[derive(Clone, Copy, PartialEq, Eq)]
struct SelectionValue(&'static str); // for syncing the selected option with the nav bar selection

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

// static app data
#[derive(Clone, Copy, PartialEq, Eq)]
enum AppData {
    SiteData(SiteData),
    ProjectsPage(Page),
    AboutPage(Page),
    ContactPage(Page),
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

async fn fetch_all_projects<G: Html>() -> Result<Repositories, reqwasm::Error> {
    let url = format!("{}{}", API_BASE_URL, "?per_page=100");
    let resp = Request::get(&url).send().await?;

    let resp_str = resp.text().await?;
    let repositories: Repositories = serde_json::from_str(&resp_str.as_str())?;
    Ok(repositories)
}

#[component]
async fn App<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    // Site data state setup
    let site_data = SiteData::builder(
        "https://www.facebook.com/BHBoruff/",
        "https://www.linkedin.com/in/benjaminboruff/",
        "https://stackoverflow.com/users/6026248/benjamin-h-boruff",
        "https://github.com/benjaminboruff",
    );

    // static app data setup for access by components
    let projects_page = Page::builder("Projects", "/", true);
    let about_page = Page::builder("About", "/about", false);
    let contact_page = Page::builder("Contact", "/contact", false);
    let mut static_app_data = HashMap::new();
    static_app_data.insert("site_data", AppData::SiteData(site_data));
    static_app_data.insert("projects_page", AppData::ProjectsPage(projects_page));
    static_app_data.insert("about_page", AppData::AboutPage(about_page));
    static_app_data.insert("contact_page", AppData::ContactPage(contact_page));

    let app_state_ref = create_ref(cx, static_app_data);
    provide_context_ref(cx, app_state_ref);

    // fetch github data for projects component
    let github = fetch_all_projects::<G>().await.unwrap_or_default();
    let mut repos: Vec<Repository> = github.repositories;
    const TOP_SIX_REPOS: &[&str] = &[
        "minigrep",
        "blasteroids",
        "calculator",
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
                                        info!("{}", router_path.get().0);
                                        selection_value.set(SelectionValue("Projects"));
                                        info!("{}", selection_value.get().0);
                                         view! {cx, // Projects
                                            Nav(route=router_path.get().0, select_value=selection_value.get().0){}
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
                                        info!("{}", router_path.get().0);
                                        selection_value.set(SelectionValue("About"));
                                        info!("{}", selection_value.get().0);
                                        view! {cx,
                                                Nav(route=router_path.get().0, select_value=selection_value.get().0){}
                                                About{}
                                            }
                                    },
                                    AppRoutes::Contact =>{
                                        router_path.set(TabRoute("/contact"));
                                        info!("{}", router_path.get().0);
                                        selection_value.set(SelectionValue("Contact"));
                                        info!("{}", selection_value.get().0);
                                        view!{cx,
                                            Nav(route=router_path.get().0, select_value=selection_value.get().0){}
                                            Contact{}
                                        }
                                     }
                                    ,
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
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
