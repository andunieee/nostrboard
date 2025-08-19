use leptos::prelude::*;

#[derive(Clone, Debug)]
pub enum DataValue {
    Text(String),
    Image(String),
    List(Vec<String>),
}

#[component]
pub fn DataCard(
    title: &'static str,
    values: impl Fn() -> Vec<(&'static str, DataValue)> + Send + Sync + 'static,
) -> impl IntoView {
    let (toggled_reader, toggled_writer) = signal::<Option<usize>>(None);

    view! {
        <div class="bg-black border border-purple-200 p-1 hover:border-gray-50 transition-colors w-96">
            <h3 class="text-xs font-semibold text-gray-200 uppercase tracking-wide mb-1">
                {title}
            </h3>
            <ForEnumerate
                each=values
                key=|v| v.0
                children=move |index, (k, v)| {
                    let opened = move || toggled_reader() == Some(index());
                    let closed = move || toggled_reader() != Some(index());

                    view! {
                        <div
                            class="text-sm text-gray-300 flex justify-between gap-4 hover:bg-purple-100/20"
                            class:h-5=closed
                            class:max-h-72=opened
                        >
                            <div
                                class="h-full overflow-hidden text-ellipsis w-32 cursor-pointer"
                                on:click=move |_| {
                                    toggled_writer
                                        .update(move |mut i| {
                                            *i = if *i == Some(index()) { None } else { Some(index()) };
                                        });
                                }
                            >
                                {k}
                            </div>
                            {match v {
                                DataValue::Text(text) => {
                                    view! {
                                        <div
                                            class="h-full overflow-hidden text-ellipsis"
                                            class:whitespace-pre-wrap=opened
                                            class:break-all=opened
                                        >
                                            {text.as_str()}
                                        </div>
                                    }
                                }
                                DataValue::Image(url) => {
                                    view! {
                                        <div
                                            class="h-full cursor-pointer"
                                            on:click=move |_| {
                                                toggled_writer
                                                    .update(move |mut i| {
                                                        *i = if *i == Some(index()) { None } else { Some(index()) };
                                                    });
                                            }
                                        >
                                            <img class="h-full" src=url />
                                        </div>
                                    }
                                }
                                DataValue::List(items) => {
                                    view! {
                                        <div
                                            class="h-full overflow-hidden text-ellipsis flex"
                                            class:whitespace-pre-wrap=opened
                                            class:break-all=opened
                                            class:flex-col=opened
                                            class:gap-2=closed
                                        >
                                            {items
                                                .into_iter()
                                                .map(|text| {
                                                    view! { <span>{text}</span> }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                }
                            }}
                        </div>
                    }
                }
            />
        </div>
    }
}
