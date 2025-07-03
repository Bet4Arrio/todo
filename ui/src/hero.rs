use api::data;
use dioxus::{
    html::{div, g::format, param::value},
    prelude::*,
};
// const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
#[derive(PartialEq, Clone, Props)]
struct OptionalProps {
    head: Option<String>,
    children: Option<Element>,
}
#[component]
pub fn Card(props: OptionalProps) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md",
            if let Some(head) = props.head {
                    div {
                        class: "border-b-2 border-gray-300 p-6",
                        {head}
                    }
            }
            div{
                class: "p-6",
            {props.children}
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    let mut new_task_name = use_signal(|| "".to_string());
    let mut new_task_description = use_signal(|| "".to_string());
    let mut tasks = use_signal(|| Vec::<data::Task>::new());
    rsx! {
        div {
            class:"h-screen w-full bg-gradient-to-r from-blue-500 to-purple-500",
            div {
                class: "flex h-full flex-col items-center justify-center",
                Card{
                    head: format!("nova tarefa {}",  tasks().len()),
                    div{
                        class: "mb-4 flex flex-col gap-5",

                        input {
                            class:"border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500",
                            oninput: move |event| new_task_name.set(event.value()),
                            value: new_task_name()

                        }

                        textarea {
                            class:"border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500",
                            oninput: move |event| new_task_description.set(event.value()),
                            value: new_task_description()
                        }

                        button {
                            class:"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| {
                                let id = tasks.read().len();
                                tasks.write().push(data::Task { id: id,
                                    name: new_task_name(),
                                    description: Some(new_task_description()), completed: false });
                                new_task_description.set("".to_string());
                                new_task_name.set("".to_string());
                                // Add task logic here
                            },
                            "Add"
                        }
                    }
                },
                div {
                    class:"flex flex-row w-full gap-3",
                    for task in tasks() {
                         Card{
                            head: task.name,
                            div {
                                class:"text-gray-600",
                                {task.description}
                            }
                            button {
                                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                                onclick: move |_| {
                                    tasks.write().retain(|t| t.id != task.id);
                                },
                                "delete"
                            }
                        }
                    }
                }
                // div {
                //     class: "text-4xl font-bold text-white",
                // }
            }
        }
    }
}
