// use log::info;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[derive(Props)]
pub struct NavProps {
    route: &'static str,
    select_value: &'static str,
}

#[component]
pub fn Nav<G: Html>(cx: Scope, props: NavProps) -> View<G> {
    let change_route = |page: &str| -> &str {
        match page {
            "Projects" => "/",
            "About" => "/about",
            "Contact" => "/contact",
            _ => "/",
        }
    };

    let opt_val = create_signal(cx, String::new());
    opt_val.set(props.select_value.to_string());

    match props.route {
        "/" => view! {cx,
            div {
                // select element with options when viewed on small devices
                div(class="sm:hidden") {
                    label(for="tabs", class="sr-only"){ "Select a tab" }
                    select(
                        on:change=move |_| {
                            let new_route = change_route(opt_val.get().as_str());
                            navigate(new_route);
                        },
                        bind:value=opt_val,
                        id="tabs",
                        name="tabs",
                        class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500"
                    )
                    {
                        option(selected=true, value="Projects") { "Projects" }
                        option(value="About") { "About" }
                        option(value="Contact") { "Contact" }
                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href="/", class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="projects") {
                            span { "Projects" }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/about", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "About" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/contact", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "Contact" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                    }
                }
            }
        },
        "/about" => view! {cx,
            div {
                // select element with options when viewed on small devices
                div(class="sm:hidden") {
                    label(for="tabs", class="sr-only"){ "Select a tab" }
                    select(
                        on:change=move |_| {
                            let new_route = change_route(opt_val.get().as_str());
                            navigate(new_route);
                        },
                        bind:value=opt_val,
                        id="tabs",
                        name="tabs",
                        class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500"
                    )
                    {
                        option(value="Projects") { "Projects" }
                        option(selected=true, value="About") { "About" }
                        option(value="Contact") { "Contact" }

                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href="/", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "Projects" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/about", class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="about") {
                            span { "About" }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/contact", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "Contact" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                    }
                }
            }
        },
        "/contact" => view! {cx,
            div {
                // select element with options when viewed on small devices
                div(class="sm:hidden") {
                    label(for="tabs", class="sr-only"){ "Select a tab" }
                    select(
                        on:change=move |_| {
                            let new_route = change_route(opt_val.get().as_str());
                            navigate(new_route);
                        },
                        bind:value=opt_val,
                        id="tabs",
                        name="tabs",
                        class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500"
                    )
                    {
                        option(value="Projects") { "Projects" }
                        option(value="About") { "About" }
                        option(selected=true, value="Contact") { "Contact" }
                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href="/", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "Projects" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/about", class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { "About" }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href="/contact", class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="contact") {
                            span { "Contact" }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                    }
                }
            }
        },
        _ => view! {cx,},
    }
}
