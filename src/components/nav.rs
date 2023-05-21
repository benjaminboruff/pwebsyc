use sycamore::prelude::*;
use sycamore_router::navigate;

#[derive(Clone, Copy, PartialEq, Eq)]
struct TabStateData {
    selected_anchor_classes: &'static str,
    unselected_anchor_classes: &'static str,
    selected_span_classes: &'static str,
    unselected_span_classes: &'static str,
}

impl TabStateData {
    fn new() -> Self {
        Self {
            selected_anchor_classes: "bg-gray-100 text-gray-900 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
            unselected_anchor_classes: "bg-gray-100 text-gray-400 hover:text-gray-700 group relative min-w-0 flex-1 overflow-hidden py-4 px-4 text-center text-sm font-medium hover:bg-sky-100 focus:z-10",
            selected_span_classes: "bg-pink-500 absolute inset-x-0 bottom-0 h-0.5",
            unselected_span_classes: "bg-transparent absolute inset-x-0 bottom-0 h-0.5",
        }
    }
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
    fn new() -> Self {
        Self {
            project_anchor_classes: "",
            project_span_classes: "",
            about_anchor_classes: "",
            about_span_classes: "",
            contact_anchor_classes: "",
            contact_span_classes: "",
        }
    }

    fn select_project(&self, tab_state_data: &TabStateData) -> Self {
        Self {
            project_anchor_classes: tab_state_data.selected_anchor_classes,
            project_span_classes: tab_state_data.selected_span_classes,
            about_anchor_classes: tab_state_data.unselected_anchor_classes,
            about_span_classes: tab_state_data.unselected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_about(&self, tab_state_data: &TabStateData) -> Self {
        Self {
            project_anchor_classes: tab_state_data.unselected_anchor_classes,
            project_span_classes: tab_state_data.unselected_span_classes,
            about_anchor_classes: tab_state_data.selected_anchor_classes,
            about_span_classes: tab_state_data.selected_span_classes,
            contact_anchor_classes: tab_state_data.unselected_anchor_classes,
            contact_span_classes: tab_state_data.unselected_span_classes,
        }
    }
    fn select_contact(&self, tab_state_data: &TabStateData) -> Self {
        Self {
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
    let tab_state_data = TabStateData::new();
    // the default route is / with the projects tab selected
    let current_tab_state = CurrentTabState::new().select_project(&tab_state_data);
    let tab_state = create_signal(cx, current_tab_state);

    let select_option = |path| select_state.set(path);
    let select_tab = move |path| match path {
        "/" => tab_state.set(current_tab_state.select_project(&tab_state_data)),
        "/about" => tab_state.set(current_tab_state.select_about(&tab_state_data)),
        "/contact" => tab_state.set(current_tab_state.select_contact(&tab_state_data)),
        _ => tab_state.set(current_tab_state.select_project(&tab_state_data)),
    };
    // set route - this resets the rout to match the default tab upon page refresh
    navigate(*select_state.get());

    view! {cx,
        div {
            // select element with options when viewed on small devices
            div(class="sm:hidden") {
                label(for="tabs", class="sr-only"){ "Select a tab" }
                select(on:click=|_| {navigate(*select_state.get())}, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                    option(on:click=move |_| {select_option("/"); select_tab("/")}) {"Projects"}
                    option(on:click=move |_| {select_option("/about"); select_tab("/about")}) {"About"}
                    option(on:click=move |_| {select_option("/contact"); select_tab("/contact")}) {"Contact"}
                }
            }
            // tab elements when viewed on medium or larger devices
            div(class="hidden sm:block") {
                nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                    a(on:click=move |_| {select_tab("/"); select_option("/")}, href="/", class=(tab_state.get().project_anchor_classes), aria-current="page") {
                        span { "Projects" }
                        span(aria-hidden="true", class=(tab_state.get().project_span_classes)) {}
                    }
                    a(on:click=move|_| {select_tab("/about"); select_option("/about")}, href="/about", class=(tab_state.get().about_anchor_classes)) {
                        span { "About" }
                        span(aria-hidden="true", class=(tab_state.get().about_span_classes)) {}
                    }
                    a(on:click=move|_| {select_tab("/contact"); select_option("/contact")}, href="/contact", class=(tab_state.get().contact_anchor_classes)) {
                        span { "Contact" }
                        span(aria-hidden="true", class=(tab_state.get().contact_span_classes)) {}
                    }
                }
            }
        }
    }
}
