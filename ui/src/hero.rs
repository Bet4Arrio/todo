use api::data;
use chrono::{DateTime, Local, Utc};
use dioxus::{
    html::{div, g::format, param::value},
    prelude::*,
};

fn get_timestamp_in_milliseconds() -> usize {
    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let milliseconds_timestamp = naive_utc.and_utc().timestamp_millis() as usize;

    milliseconds_timestamp
}

fn new_task(name: String, description: String) -> data::Task {
    data::Task {
        id: get_timestamp_in_milliseconds(),
        name: name,
        description: Some(description),
        completed: false,
    }
}

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
            class: "bg-white rounded-lg shadow-md w-128 ",
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

#[derive(PartialEq, Clone, Props)]
struct TaskCardProps {
    task: data::Task,
    remove_callback: Callback<usize, ()>,
}

#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    rsx! {
        div {
            class: "relative bg-white rounded-lg shadow-md w-128 h-64",
            div {
                class: "border-b-2 border-gray-300 p-6",
                h3{
                    class: "text-lg font-bold",
                    {props.task.name},
                }
            }
            div{
                class: "p-6 flex flex-col",
                p{
                    class: "text-gray-700",
                    "ID: {props.task.id}"
                }
                p{
                    class: "text-gray-700",
                    "Descrição: ",
                    {props.task.description}
                }
            }
            div{
                class: " absolute bottom-0 left-0 w-full  text-white p-4 rounded-b",
                div{
                    class: " w-full flex flex-row justify-evenly divide-x divide-gray-300 gap-2",
                    // button {
                    //     class:"text-red-800 w-full font-semibold py-2 px-4 focus:outline-none focus:ring-2 focus:ring-red-400",
                    //     onclick: move |_| props.remove_callback.call(props.task.id),
                    //     "Delete"
                    // }
                    button {
                        class:"text-green-800 w-full font-semibold py-2 px-4 focus:outline-none focus:ring-2 focus:ring-green-400",
                        onclick: move |_| props.remove_callback.call(props.task.id),
                        "Done"
                    }
                }
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
            class:"min-h-screen w-full py-10 bg-gradient-to-r from-blue-500 to-purple-500",
            div {
                class: "min-h-full w-full flex flex-row flex-wrap justify-start px-32 gap-3",
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
                                tasks.write().push(new_task( new_task_name(), new_task_description()));
                                new_task_description.set("".to_string());
                                new_task_name.set("".to_string());
                                // Add task logic here
                            },
                            "Add"
                        }
                    }
                },
                for task in tasks(){
                    TaskCard{
                        task: task,
                        remove_callback: move |id:usize| {
                            tasks.write().retain(|t| t.id != id);
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
