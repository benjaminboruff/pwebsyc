use sycamore::prelude::*;

#[component]
pub fn Me<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div(){
            p {"Hey, I'm Ben Boruff, programmer and system administrator."}
        }
    }
}
