use crate::AboutSelected;
use crate::AppData;
use crate::ContactSelected;
use crate::CurrentTabState;
use crate::Page;
use crate::ProjectSelected;
use crate::TabStateData;
use log::info;
use std::collections::HashMap;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
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
    let value = create_signal(cx, String::new());
    value.set("/".to_string());

    // set initial route - this resets the rout to match the default tab upon page refresh
    // which is typically the root / to list all the projects
    navigate(value.get().as_str());

    view! {cx,
        div {
            // select element with options when viewed on small devices
            div(class="sm:hidden") {
                label(for="tabs", class="sr-only"){ "Select a tab" }
                select(on:change=move |_| {navigate(value.get().as_str()); info!("{}", value)}, bind:value=value, id="tabs", name="tabs", class="block w-full border-gray-300 focus:border-pink-500 focus:ring-pink-500") {
                    option(selected=projects_selected.get().value(), value="/", on:click=move |_| { click_projects_tab_or_option() }) { (projects_page.name) }
                    option(selected=about_selected.get().value(), value="/about", on:click=move |_| { click_about_tab_or_option() }) { (about_page.name) }
                    option(selected=contact_selected.get().value(), value="/contact",on:click=move |_| { click_contact_tab_or_option()}) { (contact_page.name) }

                }
            }
            // tab elements when viewed on medium or larger devices
            div(class="hidden sm:block") {
                nav(class="isolate flex divide-x divide-gray-300 shadow-md", aria-label="Tabs") {
                    a(on:click=move |_| { click_projects_tab_or_option() }, href=(projects_page.route), class=(tab_state.get().project_anchor_classes), aria-current="page") {
                        span { (projects_page.name) }
                        span(aria-hidden="true", class=(tab_state.get().project_span_classes)) {}
                    }
                    a(on:click=move|_| { click_about_tab_or_option() }, href=(about_page.route), class=(tab_state.get().about_anchor_classes)) {
                        span { (about_page.name) }
                        span(aria-hidden="true", class=(tab_state.get().about_span_classes)) {}
                    }
                    a(on:click=move|_| { click_contact_tab_or_option() }, href=(contact_page.route), class=(tab_state.get().contact_anchor_classes)) {
                        span { (contact_page.name) }
                        span(aria-hidden="true", class=(tab_state.get().contact_span_classes)) {}
                    }
                }
            }
        }
    }
}
