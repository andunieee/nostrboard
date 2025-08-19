use leptos::prelude::*;

mod basic_relays;
mod datacard;
mod metadata;
mod pool;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init();
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let pubkey = ritual::PubKey::from_hex(
        "cba13a6ea15012c099409259345f34c76a1b9b4d8e301075a580eb8c20bc5e5d",
    )
    .unwrap();

    view! {
        <div class="min-h-screen font-mono text-sm">
            <header class="border-b border-purple-200 bg-purple-400/20">
                <div class="mx-auto px-4 py-4">
                    <h1 class="text-2xl font-bold font-sans">"Nostr Dashboard"</h1>
                </div>
            </header>
            <main class="mx-auto px-4 py-6 flex flex-wrap gap-2">
                <metadata::MetadataSection pubkey=pubkey />
                <basic_relays::RelayListSection pubkey=pubkey />
            </main>
        </div>
    }
}
