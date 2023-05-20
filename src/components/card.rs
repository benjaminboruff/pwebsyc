use sycamore::prelude::*;

#[derive(Props)]
pub struct CardProps<'a, G: Html> {
    children: Children<'a, G>,
    item: i32,
}

#[component]
pub fn Card<'a, G: Html>(cx: Scope<'a>, props: CardProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! {cx,
        div(class="border rounded-lg m-2 p-2 bg-sky-50 shadow-md md:w-1/2 md:mx-auto"){
            div(class="p-1"){
                p(class="text-center") {"Project: " (props.item)}
                (children)
            }

        }
    }
}
