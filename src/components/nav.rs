use crate::components::option::Option;
use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::CurrentTabState;
use crate::SelectState;
use crate::TabStateData;

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
    let select_state: &Signal<SelectState> = use_context(cx);
    let tab_state_data: &TabStateData = use_context(cx);
    let tab_state: &Signal<CurrentTabState> = use_context(cx);
    let current_tab_state = *tab_state.get();

    let select_option = |path| select_state.set(path);
    let select_tab = move |path| match path {
        "/" => tab_state.set(current_tab_state.select_project(&tab_state_data)),
        "/about" => tab_state.set(current_tab_state.select_about(&tab_state_data)),
        "/contact" => tab_state.set(current_tab_state.select_contact(&tab_state_data)),
        _ => tab_state.set(current_tab_state.select_project(&tab_state_data)),
    };
    // set initial route - this resets the rout to match the default tab upon page refresh
    // which is typically the root / to list all the projects
    navigate(select_state.get().path());

    view! {cx,
        div {
            // select element with options when viewed on small devices
            div(class="sm:hidden") {
                label(for="tabs", class="sr-only"){ "Select a tab" }
                select(on:click=|_| {navigate(select_state.get().path())}, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                    option(on:click=move |_| {select_option(SelectState("/")); select_tab("/")}) {"Projects"}
                    // option(on:click=move |_| {select_option(SelectState("/about")); select_tab("/about")}) {"About"}
                    Option(selected=create_signal(cx, true), label="About", path="/about")
                    option(on:click=move |_| {select_option(SelectState("/contact")); select_tab("/contact")}) {"Contact"}

                }
            }
            // tab elements when viewed on medium or larger devices
            div(class="hidden sm:block") {
                nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                    a(on:click=move |_| {select_tab("/"); select_option(SelectState("/"))}, href="/", class=(tab_state.get().project_anchor_classes), aria-current="page") {
                        span { "Projects" }
                        span(aria-hidden="true", class=(tab_state.get().project_span_classes)) {}
                    }
                    a(on:click=move|_| {select_tab("/about"); select_option(SelectState("/about"))}, href="/about", class=(tab_state.get().about_anchor_classes)) {
                        span { "About" }
                        span(aria-hidden="true", class=(tab_state.get().about_span_classes)) {}
                    }
                    a(on:click=move|_| {select_tab("/contact"); select_option(SelectState("/contact"))}, href="/contact", class=(tab_state.get().contact_anchor_classes)) {
                        span { "Contact" }
                        span(aria-hidden="true", class=(tab_state.get().contact_span_classes)) {}
                    }
                }
            }
        }
    }
}
