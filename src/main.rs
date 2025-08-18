use std::sync::LazyLock;

use leptos::{prelude::*, reactive::spawn_local};
use ritual::{Filter, Metadata, Occurrence, PubKey};

static POOL: LazyLock<ritual::Pool> =
    LazyLock::new(|| ritual::Pool::new(ritual::PoolOptions::default()));

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init();
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let pubkey =
        PubKey::from_hex("cba13a6ea15012c099409259345f34c76a1b9b4d8e301075a580eb8c20bc5e5d")
            .unwrap();

    view! {
        <div class="min-h-screen font-mono text-sm">
            <header class="border-b border-purple-200 bg-purple-400/20">
                <div class="mx-auto px-4 py-4">
                    <h1 class="text-2xl font-bold font-sans">"Nostr Dashboard"</h1>
                </div>
            </header>
            <main class="mx-auto px-4 py-6">
                <MetadataSection pubkey=pubkey />
            </main>
        </div>
    }
}

#[component]
fn MetadataSection(pubkey: ritual::PubKey) -> impl IntoView {
    let (metadata_reader, metadata_writer) = signal::<Option<Metadata>>(None);

    let filter = Filter {
        kinds: Some(vec![0.into()]),
        authors: Some(vec![pubkey.clone()]),
        ..Default::default()
    };

    let _ = Effect::new(move || {
        let filter = filter.clone();

        spawn_local(async move {
            let mut occurrences = POOL
                .subscribe(
                    vec![
                        "purplepag.es".to_string(),
                        "relay.nos.social".to_string(),
                        "relay.primal.net".to_string(),
                        "relay.damus.io".to_string(),
                    ],
                    filter,
                    ritual::SubscriptionOptions::default(),
                )
                .await;

            while let Some(occ) = occurrences.recv().await {
                match occ {
                    Occurrence::Event(event) => match ritual::Metadata::from_event(&event) {
                        Ok(metadata) => {
                            metadata_writer.set(Some(metadata));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    });

    let values = move || {
        let metadata_read = metadata_reader.get();
        let mut values = vec![
            ("hex public key", DataValue::Text(pubkey.to_hex())),
            ("npub", DataValue::Text(pubkey.to_npub())),
        ];

        match metadata_read {
            None => {}
            Some(metadata) => {
                if let Some(name) = &metadata.name {
                    values.push(("name", DataValue::Text(name.clone())));
                }
                if let Some(picture) = &metadata.picture {
                    values.push(("picture", DataValue::Image(picture.clone())));
                }
                if let Some(about) = &metadata.about {
                    values.push(("about", DataValue::Text(about.clone())));
                }
                if let Some(banner) = &metadata.banner {
                    values.push(("banner", DataValue::Image(banner.clone())));
                }
                if let Some(website) = &metadata.website {
                    values.push(("website", DataValue::Text(website.clone())));
                }
                if let Some(display_name) = &metadata.display_name {
                    values.push(("display_name", DataValue::Text(display_name.clone())));
                }
                if let Some(nip05) = &metadata.nip05 {
                    values.push(("nip05", DataValue::Text(nip05.clone())));
                }
                if let Some(lud16) = &metadata.lud16 {
                    values.push(("lud16", DataValue::Text(lud16.clone())));
                }
            }
        };

        values
    };

    view! {
        <>
            <DataCard title="ACCOUNT DATA" values=values />
        </>
    }
}

#[derive(Clone, Debug)]
enum DataValue {
    Text(String),
    Image(String),
}

#[component]
fn DataCard(
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
                            }}
                        </div>
                    }
                }
            />
        </div>
    }
}
