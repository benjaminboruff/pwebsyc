use crate::Repository;
use sycamore::prelude::*;

#[derive(Props)]
pub struct CardProps {
    repo: Repository,
}

#[component]
pub fn Card<G: Html>(cx: Scope, props: CardProps) -> View<G> {
    view! {cx,
        div(class="border rounded-lg m-2 p-2 bg-sky-50 shadow-md md:w-1/2 md:mx-auto"){
            div(class="prose"){
                p(class="text-center") {a(href=props.repo.html_url, class="no-underline"){ (props.repo.name) } }
                p(class="text-center") {(props.repo.description)}
            }

        }
    }
}
