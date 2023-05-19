use sycamore::prelude::*;

#[derive(Prop)]
pub struct MeProps<'a, G: Html> {
    children: Children<'a, G>,
    item: i32,
}

#[component]
pub fn Me<'a, G: Html>(cx: Scope<'a>, props: MeProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! {cx,
        div(){
            (props.item)
            p {"Hey, I'm Ben Boruff, programmer and system administrator."}
            (children)
        }
    }
}
