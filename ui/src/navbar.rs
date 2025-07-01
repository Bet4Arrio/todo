use dioxus::prelude::*;
#[component]
pub fn NavLink(children: Element) -> Element {
    rsx! {
        {children}
    }
}
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {

        nav {
            class: "bg-blue-600 px-4 py-3 flex items-center justify-between",
            id: "navbar",
            div {
                class: "text-white font-bold text-xl",
                "Todo App"
            }
            {children}
        }
    }
}
