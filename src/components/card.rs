use sycamore::prelude::*;

#[derive(Prop)]
pub struct CardProps<'a, G: Html> {
    children: Children<'a, G>,
    item: i32,
}

#[component]
pub fn Card<'a, G: Html>(cx: Scope<'a>, props: CardProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! {cx,
        div(class="border rounded-lg my-2 bg-sky-100 shadow-lg"){
            article(class="p-1"){
                (props.item)
                p {"Hey, I'm Ben Boruff, programmer and system administrator."}
                (children)
            }

        }
    }
}