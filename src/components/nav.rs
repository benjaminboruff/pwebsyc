use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
    let select_state = create_signal(cx, "/");

    view! {cx,
        div(class="container mx-auto") {
            div(class="sm:hidden") {
                label(for="tabs", class="sr-only"){ "Select a tab" }
                select(on:click=|_| {navigate(*select_state.get())}, id="tabs", name="tabs", class="block w-full rounded-lg border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                    option(on:click=|_| {select_state.set("/")}) {"Projects"}
                    option(on:click=|_| {select_state.set("/about")}) {"About"}
                    option(on:click=|_| {select_state.set("/contact")}) {"Contact"}
                }
            }
            div(class="hidden sm:block") {
                nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                    a(href="/", class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10", aria-current="page") {
                        span { "Projects" }
                        span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                    }
                    a(href="/about", class="bg-gray-100 text-gray-500 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10") {
                        span { "About" }
                        span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                    }
                    a(href="/contact", class="bg-gray-100 text-gray-500 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10") {
                        span { "Contact" }
                        span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                    }
                }
            }
        }
    }
}
