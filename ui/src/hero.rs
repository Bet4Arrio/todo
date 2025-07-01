use dioxus::{html::div, prelude::*};

// const HERO_CSS: Asset = asset!("/assets/styling/hero.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class:"h-screen w-full bg-gradient-to-r from-blue-500 to-purple-500",
            div {
                class: "flex h-full flex-col items-center justify-center",
                div {
                    class: "text-4xl font-bold text-white",
                    "Welcome to my todo list!"
                }
            }
        }
    }
}
