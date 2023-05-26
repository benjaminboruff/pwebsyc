use crate::AboutSelected;
use crate::AppData;
use crate::ContactSelected;
use crate::CurrentTabState;
use crate::Page;
use crate::ProjectSelected;
use crate::TabStateData;
// use log::info;
use std::collections::HashMap;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[derive(Props)]
pub struct NavProps {
    route: &'static str,
}

#[component]
pub fn Nav<G: Html>(cx: Scope, props: NavProps) -> View<G> {
    let static_app_data: &HashMap<&str, AppData> = use_context(cx);
    let projects_page_state_data = *static_app_data.get("projects_page").unwrap();
    let about_page_state_data = *static_app_data.get("about_page").unwrap();
    let contact_page_state_data = *static_app_data.get("contact_page").unwrap();

    let projects_page = if let AppData::ProjectsPage(data) = projects_page_state_data {
        data
    } else {
        Page::new()
    };

    let about_page = if let AppData::AboutPage(data) = about_page_state_data {
        data
    } else {
        Page::new()
    };

    let contact_page = if let AppData::ContactPage(data) = contact_page_state_data {
        data
    } else {
        Page::new()
    };

    let projects_selected: &Signal<ProjectSelected> = use_context(cx);
    let about_selected: &Signal<AboutSelected> = use_context(cx);
    let contact_selected: &Signal<ContactSelected> = use_context(cx);
    let tab_state_data: &TabStateData = use_context(cx);
    let tab_state: &Signal<CurrentTabState> = use_context(cx);

    let click_projects_tab_or_option = move || {
        projects_selected.set(ProjectSelected(true));
        about_selected.set(AboutSelected(false));
        contact_selected.set(ContactSelected(false));
        tab_state.set(tab_state.get().select_project(&tab_state_data));
    };
    let click_about_tab_or_option = move || {
        projects_selected.set(ProjectSelected(false));
        about_selected.set(AboutSelected(true));
        contact_selected.set(ContactSelected(false));
        tab_state.set(tab_state.get().select_about(&tab_state_data));
    };
    let click_contact_tab_or_option = move || {
        projects_selected.set(ProjectSelected(false));
        about_selected.set(AboutSelected(false));
        contact_selected.set(ContactSelected(true));
        tab_state.set(tab_state.get().select_contact(&tab_state_data));
    };
    let select_value = create_signal(cx, String::new());
    select_value.set("/".to_string());

    match props.route {
        "/" => view! {cx,
            div {
                // select element with options when viewed on small devices
                div(class="sm:hidden") {
                    label(for="tabs", class="sr-only"){ "Select a tab" }
                    select(on:change=move |_| {navigate(select_value.get().as_str())}, bind:value=select_value, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                        option(selected=projects_selected.get().value(), value="/", on:click=move |_| { click_projects_tab_or_option() }) { (projects_page.name) }
                        option(selected=about_selected.get().value(), value="/about", on:click=move |_| { click_about_tab_or_option() }) { (about_page.name) }
                        option(selected=contact_selected.get().value(), value="/contact",on:click=move |_| { click_contact_tab_or_option()}) { (contact_page.name) }

                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href=(projects_page.route), class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="projects") {
                            span { (projects_page.name) }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(about_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { (about_page.name) }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(contact_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { (contact_page.name) }
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
                    select(on:change=move |_| {navigate(select_value.get().as_str())}, bind:value=select_value, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                        option(selected=projects_selected.get().value(), value="/", on:click=move |_| { click_projects_tab_or_option() }) { (projects_page.name) }
                        option(selected=about_selected.get().value(), value="/about", on:click=move |_| { click_about_tab_or_option() }) { (about_page.name) }
                        option(selected=contact_selected.get().value(), value="/contact",on:click=move |_| { click_contact_tab_or_option()}) { (contact_page.name) }

                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href=(projects_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { (projects_page.name) }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(about_page.route), class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="about") {
                            span { (about_page.name) }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(contact_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { (contact_page.name) }
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
                    select(on:change=move |_| {navigate(select_value.get().as_str())}, bind:value=select_value, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                        option(selected=projects_selected.get().value(), value="/", on:click=move |_| { click_projects_tab_or_option() }) { (projects_page.name) }
                        option(selected=about_selected.get().value(), value="/about", on:click=move |_| { click_about_tab_or_option() }) { (about_page.name) }
                        option(selected=contact_selected.get().value(), value="/contact",on:click=move |_| { click_contact_tab_or_option()}) { (contact_page.name) }

                    }
                }
                // tab elements when viewed on medium or larger devices
                div(class="hidden sm:block") {
                    nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                        a(href=(projects_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="page") {
                            span { (projects_page.name) }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(about_page.route), class="bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10") {
                            span { (about_page.name) }
                            span(aria-hidden="true", class="bg-transparent absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                        a(href=(contact_page.route), class="bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-md font-medium hover:bg-sky-100 focus:z-10", aria-current="contact") {
                            span { (contact_page.name) }
                            span(aria-hidden="true", class="bg-pink-500 absolute inset-x-0 bottom-0 h-0.5") {}
                        }
                    }
                }
            }
        },
        _ => view! {cx,},
    }
}
