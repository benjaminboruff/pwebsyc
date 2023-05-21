use sycamore::prelude::*;
use sycamore_router::navigate;

#[derive(Clone, Copy, PartialEq, Eq)]
struct TabStateData {
    selected_anchor_classes: &'static str,
    unselected_anchor_classes: &'static str,
    selected_span_classes: &'static str,
    unselected_span_classes: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct CurrentTabState {
    project_anchor_classes: &'static str,
    project_span_classes: &'static str,
    about_anchor_classes: &'static str,
    about_span_classes: &'static str,
    contact_anchor_classes: &'static str,
    contact_span_classes: &'static str,
}

impl CurrentTabState {
    fn select_project(self, tab_state_data: &TabStateData) -> CurrentTabState {
        CurrentTabState {
            project_anchor_classes: tab_state_data.selected_anchor_classes,
            project_span_classes: tab_state_data.selected_span_classes,
            about_anchor_classes: tab_state_data.unselected_anchor_classes,
            about_span_classes: tab_state_data.unselected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_about(self, tab_state_data: &TabStateData) -> CurrentTabState {
        CurrentTabState {
            project_anchor_classes: tab_state_data.unselected_anchor_classes,
            project_span_classes: tab_state_data.unselected_span_classes,
            about_anchor_classes: tab_state_data.selected_anchor_classes,
            about_span_classes: tab_state_data.selected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_contact(self, tab_state_data: &TabStateData) -> CurrentTabState {
        CurrentTabState {
            project_anchor_classes: tab_state_data.unselected_anchor_classes,
            project_span_classes: tab_state_data.unselected_span_classes,
            about_anchor_classes: tab_state_data.unselected_anchor_classes,
            about_span_classes: tab_state_data.unselected_span_classes,
            contact_anchor_classes: tab_state_data.selected_anchor_classes,
            contact_span_classes: tab_state_data.selected_span_classes,
        }
    }
}

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
    let select_state = create_signal(cx, "/");
    let tab_state_data = TabStateData {
        selected_anchor_classes: "bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
        unselected_anchor_classes: "bg-gray-100 text-gray-500 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
        selected_span_classes: "bg-pink-500 absolute inset-x-0 bottom-0 h-0.5",
        unselected_span_classes: "bg-transparent absolute inset-x-0 bottom-0 h-0.5",
    };
    let current_tab_state = CurrentTabState {
        project_anchor_classes: tab_state_data.selected_anchor_classes,
        project_span_classes: tab_state_data.selected_span_classes,
        about_anchor_classes: tab_state_data.unselected_anchor_classes,
        about_span_classes: tab_state_data.unselected_span_classes,
        contact_anchor_classes: tab_state_data.unselected_anchor_classes,
        contact_span_classes: tab_state_data.unselected_span_classes,
    };
    let tab_state = create_signal(cx, current_tab_state);

    view! {cx,
        div {
            div(class="sm:hidden") {
                label(for="tabs", class="sr-only"){ "Select a tab" }
                select(on:click=|_| {navigate(*select_state.get())}, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                    option(on:click=|_| {select_state.set("/")}) {"Projects"}
                    option(on:click=|_| {select_state.set("/about")}) {"About"}
                    option(on:click=|_| {select_state.set("/contact")}) {"Contact"}
                }
            }
            div(class="hidden sm:block") {
                nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                    a(on:click=move|_| {tab_state.set(current_tab_state.select_project(&tab_state_data))}, href="/", class=(tab_state.get().project_anchor_classes), aria-current="page") {
                        span { "Projects" }
                        span(aria-hidden="true", class=(tab_state.get().project_span_classes)) {}
                    }
                    a(on:click=move|_| {tab_state.set(current_tab_state.select_about(&tab_state_data))}, href="/about", class=(tab_state.get().about_anchor_classes)) {
                        span { "About" }
                        span(aria-hidden="true", class=(tab_state.get().about_span_classes)) {}
                    }
                    a(on:click=move|_| {tab_state.set(current_tab_state.select_contact(&tab_state_data))}, href="/contact", class=(tab_state.get().contact_anchor_classes)) {
                        span { "Contact" }
                        span(aria-hidden="true", class=(tab_state.get().contact_span_classes)) {}
                    }
                }
            }
        }
    }
}
