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
    fn new() -> Self {
        Self {
            name: "Default",
            route: "/",
            selected: false,
        }
    }
    fn builder(name: &'static str, route: &'static str, selected: bool) -> Self {
        Self {
            name,
            route,
            selected,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct TabRoute(&'static str);

#[derive(Clone, Copy, PartialEq, Eq)]
struct ProjectSelected(bool); // state for the project selected attribute
impl ProjectSelected {
    pub fn value(&self) -> bool {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct AboutSelected(bool); // state for the select/option elements for small screens
impl AboutSelected {
    pub fn value(&self) -> bool {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct ContactSelected(bool); // state for the select/option elements for small screens
impl ContactSelected {
    pub fn value(&self) -> bool {
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
            selected_anchor_classes: "bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10",
            unselected_anchor_classes: "bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10",
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

// API that counts visits to the web-page
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

    // Nav state setup
    let projects_selected = create_signal(cx, ProjectSelected(true));
    provide_context_ref(cx, projects_selected);
    let about_selected = create_signal(cx, AboutSelected(false));
    provide_context_ref(cx, about_selected);
    let contact_selected = create_signal(cx, ContactSelected(false));
    provide_context_ref(cx, contact_selected);
    let tab_state_data = TabStateData::new();
    let tab_state_data_ref = create_ref(cx, tab_state_data);
    provide_context_ref(cx, tab_state_data_ref);
    let current_tab_state =
        create_signal(cx, CurrentTabState::new().select_project(&tab_state_data));
    provide_context_ref(cx, current_tab_state);

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
                                         view! {cx, // Projects
                                            Nav(route=router_path.get().0){}
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
                                        view! {cx,
                                            Nav(route=router_path.get().0){}
                                            div(class="flex flex-col justify-center items-center") {
                                                About{}
                                            }
                                            }
                                    },
                                    AppRoutes::Contact =>{
                                        router_path.set(TabRoute("/contact"));
                                        info!("{}", router_path.get().0);
                                        view!{cx,
                                            Nav(route=router_path.get().0){}
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
