use crate::{
    datacard::{DataCard, DataValue},
    pool::POOL,
};
use leptos::{prelude::*, reactive::spawn_local};

#[component]
pub fn MetadataSection(pubkey: ritual::PubKey) -> impl IntoView {
    let (metadata_reader, metadata_writer) = signal::<Option<ritual::Metadata>>(None);

    let _ = Effect::new(move || {
        let filter = ritual::Filter {
            kinds: Some(vec![0.into()]),
            authors: Some(vec![pubkey.clone()]),
            limit: Some(1),
            ..Default::default()
        };

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
                    ritual::Occurrence::Event(event, _) => {
                        match ritual::Metadata::from_event(&event) {
                            Ok(metadata) => {
                                metadata_writer.set(Some(metadata));
                            }
                            _ => {}
                        }
                    }
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

    view! { <DataCard title="ACCOUNT DATA" values=values /> }
}
