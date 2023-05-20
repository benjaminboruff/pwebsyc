use sycamore::prelude::*;

#[component]
pub fn Hero<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div( class="bg-sky-50 py-24 sm:py-32") {
            div(class="mx-auto max-w-7xl px-6 lg:px-8") {
                div(class="mx-auto max-w-2xl lg:mx-0") {
                    img(class="inline-block h-14 w-14 rounded-md", src="/static/images/me.jpeg", alt="")
                    p(class="mt-6 text-lg leading-8 text-gray-600"){"Hey, I'm Ben Boruff, programmer and Linux/FreeBSD system administrator."}
                    p(class="mt-6 text-lg leading-8 text-gray-600"){
                        "I enjoy using the " code{"rust"} " language for web and game development. I prefer using FreeBSD for server side hosting, but use Linux as my daily driver."
                    }
                }
                div (class=""){
                    div(class="sm:hidden") {
                        label(for="tabs", class="sr-only"){ "Select a tab" }
                        select(id="tabs", name="tabs", class="block w-full rounded-md border-gray-300 focus:border-indigo-500 focus:ring-indigo-500") {
                            option {"Projects"}
                            option {"About"}
                            option {"Contact"}
                        }
                    }
                    div(class="hidden sm:block") {
                        nav(class="isolate flex divide-x divide-gray-200 rounded-lg shadow", aria-label="Tabs") {
                            a(href="#", class="text-gray-900 rounded-l-lg group relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-center text-sm font-medium hover:bg-gray-50 focus:z-10", aria-current="page") {
                                span { "Projects" }
                                span(aria-hidden="true", class="bg-indigo-500 absolute inset-x-0 bottom-0 h-0.5") {}
                            }
                            a(href="#", class="text-gray-500 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-center text-sm font-medium hover:bg-gray-50 focus:z-10") {
                                span { "About" }
                                span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                            }
                            a(href="#", class="text-gray-500 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-center text-sm font-medium hover:bg-gray-50 focus:z-10") {
                                span { "Contact" }
                                span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                            }
                        }
                    }
                }
            }
        }
    }
}
