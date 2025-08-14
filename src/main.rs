use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white text-black font-mono">
            <Header/>
            <main class="container mx-auto px-4 py-6">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    <MarketSection/>
                    <BlockchainSection/>
                    <MiningSection/>
                    <NetworkSection/>
                    <SupplySection/>
                    <NewsSection/>
                </div>
            </main>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-gray-200 bg-white">
            <div class="container mx-auto px-4 py-4">
                <h1 class="text-2xl font-bold text-black">"Bitcoin Dashboard"</h1>
                <p class="text-sm text-gray-600 mt-1">"Real-time Bitcoin network statistics and market data"</p>
            </div>
        </header>
    }
}

#[component]
fn DataCard(title: &'static str, value: String, subtitle: Option<String>) -> impl IntoView {
    view! {
        <div class="bg-white border border-gray-200 p-4 hover:border-gray-300 transition-colors">
            <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">{title}</h3>
            <div class="text-lg font-bold text-black">{value}</div>
            {subtitle.map(|s| view! {
                <div class="text-xs text-gray-500 mt-1">{s}</div>
            })}
        </div>
    }
}

#[component]
fn MarketSection() -> impl IntoView {
    view! {
        <>
            <DataCard title="PRICE" value="$97,234.12".to_string() subtitle=Some("+2.34% (24h)".to_string())/>
            <DataCard title="MARKET CAP" value="$1.92T".to_string() subtitle=None/>
            <DataCard title="24H VOLUME" value="$47.2B".to_string() subtitle=None/>
            <DataCard title="24H HIGH" value="$98,150.00".to_string() subtitle=None/>
            <DataCard title="24H LOW" value="$95,420.30".to_string() subtitle=None/>
        </>
    }
}

#[component]
fn BlockchainSection() -> impl IntoView {
    view! {
        <>
            <DataCard title="BLOCK HEIGHT" value="875,432".to_string() subtitle=Some("2 min ago".to_string())/>
            <DataCard title="DIFFICULTY" value="103.92T".to_string() subtitle=Some("+1.2% next adj".to_string())/>
            <DataCard title="MEMPOOL SIZE" value="234.5 MB".to_string() subtitle=Some("12,456 tx".to_string())/>
            <DataCard title="AVG BLOCK TIME" value="9.8 min".to_string() subtitle=Some("last 24h".to_string())/>
        </>
    }
}

#[component]
fn MiningSection() -> impl IntoView {
    view! {
        <>
            <DataCard title="HASH RATE" value="756.2 EH/s".to_string() subtitle=Some("7-day avg".to_string())/>
            <DataCard title="NEXT DIFFICULTY" value="+1.2%".to_string() subtitle=Some("in 1,234 blocks".to_string())/>
            <DataCard title="MINING REVENUE" value="$42.3M".to_string() subtitle=Some("per day".to_string())/>
        </>
    }
}

#[component]
fn NetworkSection() -> impl IntoView {
    view! {
        <>
            <DataCard title="NODES" value="16,847".to_string() subtitle=Some("reachable".to_string())/>
            <DataCard title="LIGHTNING NODES" value="12,456".to_string() subtitle=Some("capacity: 4,892 BTC".to_string())/>
            <DataCard title="UNCONFIRMED TX" value="12,456".to_string() subtitle=Some("avg fee: 24 sat/vB".to_string())/>
        </>
    }
}

#[component]
fn SupplySection() -> impl IntoView {
    view! {
        <>
            <DataCard title="CIRCULATING SUPPLY" value="19,812,456 BTC".to_string() subtitle=Some("94.3% of total".to_string())/>
            <DataCard title="NEXT HALVING" value="2028".to_string() subtitle=Some("~1,456 days".to_string())/>
            <DataCard title="BLOCK REWARD" value="3.125 BTC".to_string() subtitle=None/>
        </>
    }
}

#[component]
fn NewsSection() -> impl IntoView {
    view! {
        <div class="col-span-full bg-white border border-gray-200 p-4">
            <h3 class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-4">"RECENT NEWS"</h3>
            <div class="space-y-3">
                <div class="border-b border-gray-100 pb-2">
                    <a href="#" class="text-blue-600 hover:text-blue-800 text-sm font-medium">
                        "Bitcoin ETF sees record inflows amid institutional adoption"
                    </a>
                    <p class="text-xs text-gray-500 mt-1">"2 hours ago - CoinDesk"</p>
                </div>
                <div class="border-b border-gray-100 pb-2">
                    <a href="#" class="text-blue-600 hover:text-blue-800 text-sm font-medium">
                        "Lightning Network capacity reaches new all-time high"
                    </a>
                    <p class="text-xs text-gray-500 mt-1">"4 hours ago - Bitcoin Magazine"</p>
                </div>
                <div>
                    <a href="#" class="text-blue-600 hover:text-blue-800 text-sm font-medium">
                        "Mining difficulty adjustment expected to increase by 1.2%"
                    </a>
                    <p class="text-xs text-gray-500 mt-1">"6 hours ago - The Block"</p>
                </div>
            </div>
        </div>
    }
}
