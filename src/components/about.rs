use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div(class="container mx-auto my-4") {
            div (class="flex flex-col justify-center items-center divide-y my-4 mx-6"){
                div(class="prose  mb-6") {
                    p(class="text-lg leading-8 text-gray-600") {
                        "I am a programmer and a
                        Linux and FreeBSD system administrator."
                    }
                    p(class="text-lg leading-8 text-gray-600") {
                        "My current interests include rust and zig related technologies. I first started programming in C in the early '90s. While I have spent most of my
                        career doing web development using interpreted languages, it's the low level stuff that I really find interesting and challenging."
                    }
                    p(class="text-lg leading-8 text-gray-600") {
                        "I also have an interest in assembly, but only have classroom experience on the M68k ... an Atari ST!"
                    }
                    p(class="text-lg leading-8 text-gray-600") {
                        "I played around a little with the MIPS ISA, but from what I have read about current architectures leads me to think the RISC-V ISA looks promising ..."
                    }

                }
                div(class="container") {
                    div(class="text-lg font-medium text-gray-600 text-center mt-7 mb-4") {
                        "Education"
                    }
                    div(class="flex md:flex-row flex-wrap justify-center items-center gap-4 md:ml-16") {
                        div(class="text-lg text-gray-600 w-60") {
                            dl {
                                dt(class="font-semibold") {"Indiana University"}
                                dt(class="font-medium my-1") {"Bachelor of Arts"}
                                dt {"Major: Philosophy"}
                                dt {"Minor: Computer Science"}
                                dt {"08-31-1995"}
                            }
                        }
                        div(class="text-lg text-gray-600 w-60") {
                            dl {
                                dt(class="font-semibold") {"Indiana University"}
                                dt(class="font-medium my-1") {"Bachelor of General Studies With Distinction"}
                                dt {"Major: General Studies - Arts and Humanities Track"}
                                dt {"08-31-2004"}
                            }
                        }
                        div(class="text-lg text-gray-600 w-60") {
                            dl {
                                dt(class="font-semibold") {"Indiana University"}
                                dt(class="font-medium my-1") {"Master of Science in Kinesiology"}
                                dt {"Major: Applied Sport Science"}
                                dt {"05-07-2010"}
                            }
                        }
                    }
                    div(class="text-lg text-gray-600 mt-4 text-center") {
                        dl() {
                            dt() {"Undergraduate GPA : 3.560"}
                            dt() {"Graduate GPA : 3.980"}
                        }
                    }
                }
            }
        }
    }
}
