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
                log::info!("got occurrence: {:?}", occ);
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
        log::info!("new: {:?}", metadata_read);
        let mut values = vec![
            ("hex public key", pubkey.to_hex()),
            ("npub", pubkey.to_npub()),
        ];

        match metadata_read {
            None => {}
            Some(metadata) => {
                if let Some(name) = &metadata.name {
                    values.push(("name", name.clone()));
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

#[component]
fn DataCard(
    title: &'static str,
    values: impl Fn() -> Vec<(&'static str, String)>,
) -> impl IntoView {
    view! {
        <div class="bg-black border border-purple-200 p-1 hover:border-gray-50 transition-colors">
            <h3 class="text-xs font-semibold text-gray-200 uppercase tracking-wide mb-1">
                {title}
            </h3>
            {values()
                .into_iter()
                .map(|(k, v)| {
                    view! {
                        <div class="text-sm text-gray-300 flex justify-between">
                            <div>{k}</div>
                            <div>{v.as_str()}</div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
