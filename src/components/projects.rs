use crate::components::card::Card;
use log::info;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

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
    let url = format!("{}", API_BASE_URL);
    let resp = Request::get(&url).send().await?;

    let resp_str = resp.text().await?;
    let repositories: Repositories = serde_json::from_str(&resp_str.as_str())?;
    Ok(repositories)
}

#[component]
pub async fn Projects<'cx, G: Html>(cx: Scope<'cx>) -> View<G> {
    let github = fetch_all_projects::<G>().await.unwrap_or_default();
    let repos = github.repositories;
    info!("{:?}", repos);
    let projects = create_signal(cx, repos);
    view! {cx,
        Keyed(
            iterable=projects,
            view=|cx, repo| view! { cx,
                Card(repo=repo){}
            },
            key=|repo| repo.clone(),
        )
    }
}
