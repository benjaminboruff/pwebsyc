use crate::components::card::Card;
use log::info;
use sycamore::prelude::*;

use crate::Repository;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    let repos_data: &Signal<Vec<Repository>> = use_context(cx);
    info!("{:?}", repos_data);
    view! {cx,
        Keyed(
            iterable=repos_data,
            view=|cx, repo| view! { cx,
                Card(repo=repo){}
            },
            key=|repo| repo.clone(),
        )
    }
}
