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
        <div class="min-h-screen bg-white text-black font-mono text-sm">
            <header class="border-b border-gray-200 bg-white">
                <div class="mx-auto px-4 py-4">
                    <h1 class="text-2xl font-bold text-black">"Nostr Dashboard"</h1>
                </div>
            </header>
            <main class="mx-auto px-4 py-6">
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-7 xl:grid-cols-10 gap-4">
                    <MetadataSection pubkey=pubkey />
                </div>
            </main>
        </div>
    }
}

#[component]
fn MetadataSection(pubkey: ritual::PubKey) -> impl IntoView {
    let (metadata, set_metadata) = signal::<Option<Metadata>>(None);

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
                        Ok(metadata) => set_metadata.set(Some(metadata)),
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    });

    view! {
        <>
            <DataCard title="PUBKEY" value=pubkey.to_hex() subtitle=Some(pubkey.to_npub()) />
            <Show when=move || { metadata.get().is_some() }>
                <DataCard
                    title="NAME"
                    value=metadata.get().unwrap().name.unwrap_or("".to_string())
                    subtitle=None
                />
                <DataCard
                    title="ABOUT"
                    value=metadata.get().unwrap().about.unwrap_or("".to_string())
                    subtitle=None
                />
            </Show>
        </>
    }
}

#[component]
fn DataCard(title: &'static str, value: String, subtitle: Option<String>) -> impl IntoView {
    view! {
        <div class="bg-white border border-gray-200 p-1 hover:border-gray-300 transition-colors">
            <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">
                {title}
            </h3>
            <div class="text-sm text-black">{value}</div>
            {subtitle.map(|s| view! { <div class="text-xs text-gray-500 mt-1">{s}</div> })}
        </div>
    }
}
