use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div (class="flex flex-col justify-center items-center"){
            div(class="prose") {
                p(class="text-lg leading-8 text-gray-600") {
                    "I am a programmer with
                    a background in front-end and back-end web development, as well as
                    Linux and FreeBSD system administration."
                }
                p(class="text-lg leading-8 text-gray-600") {
                    "My current interests are Jamstack related technologies. I have been using
                    static site generators such as 11ty, Hugo, Jigsaw
                    on my recent projects. The Jamstack philosophy of decoupling the front-end from the back-end, using
                    only APIs to connect them -- and typically using headless CMSs as the back-end, is a breath of fresh air."
                }
                p(class="text-lg leading-8 text-gray-600") {
                    "Don't get me wrong, I love back-end frameworks like Laravel, Rails,
                    and Express, but when the front-end and back-end are so tightly coupled the project can become
                    complicated very quickly. And yes, you can certainly use these back-ends to only build APIs, but that is not the typical use-case in my experience
                    working as a contract programmer with clients who want an MVP quickly."
                }
            }
        }
    }
}
