use sycamore::prelude::*;

#[component(inline_props)]
pub fn Option<'a, G: Html>(
    cx: Scope<'a>,
    selected: &'a ReadSignal<bool>,
    label: &'static str,
    path: &'static str,
) -> View<G> {
    // let select_option = |path| select_state.set(path);
    // let select_tab = move |path| match path {
    //     "/" => tab_state.set(current_tab_state.select_project(&tab_state_data)),
    //     "/about" => tab_state.set(current_tab_state.select_about(&tab_state_data)),
    //     "/contact" => tab_state.set(current_tab_state.select_contact(&tab_state_data)),
    //     _ => tab_state.set(current_tab_state.select_project(&tab_state_data)),
    // };
    view! {cx,
        // option(selected=*selected.get(), on:click=move |_| {select_option((path)); select_tab((path))}){(label)}
    }
}
