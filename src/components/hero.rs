use sycamore::prelude::*;

#[component]
pub fn Hero<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div( class="bg-sky-50 py-24 sm:py-32") {
            div(class="mx-auto max-w-3xl px-6 lg:px-3") {
                div(class="mx-auto max-w-2xl ") {
                    img(class="inline-block h-14 w-14 rounded-md", src="/static/images/me.jpeg", alt="")
                    p(class="mt-6 text-lg leading-8 text-gray-600"){"Hey, I'm Ben Boruff, programmer and Linux/FreeBSD system administrator."}
                    p(class="mt-6 text-lg leading-8 text-gray-600"){
                        "I enjoy using the " code{"rust"} " language for web and game development. I prefer using FreeBSD for servers, but my Linux laptop is my daily driver."
                    }
                }
            }
        }
    }
}
