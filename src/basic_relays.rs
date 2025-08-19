use crate::{
    datacard::{DataCard, DataValue},
    pool::POOL,
};
use leptos::{prelude::*, reactive::spawn_local};

#[component]
pub fn RelayListSection(pubkey: ritual::PubKey) -> impl IntoView {
    let (write_relays_reader, write_relays_writer) = signal::<Option<Vec<String>>>(None);
    let (read_relays_reader, read_relays_writer) = signal::<Option<Vec<String>>>(None);

    let _ = Effect::new(move || {
        let filter = ritual::Filter {
            kinds: Some(vec![10002.into()]),
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
                    ritual::Occurrence::Event(event, relay) => {
                        log::info!("event {:?} from {:?}", event, relay);
                        let mut read_list = Vec::with_capacity(event.tags.0.len());
                        let mut write_list = Vec::with_capacity(event.tags.0.len());
                        for tag in event.tags.iter() {
                            if tag.len() >= 2 && tag[0] == "r" {
                                if tag.len() == 2 {
                                    read_list.push(tag[1].clone());
                                    write_list.push(tag[1].clone());
                                } else if tag[2] == "write" {
                                    write_list.push(tag[1].clone());
                                } else if tag[2] == "read" {
                                    read_list.push(tag[1].clone());
                                }
                            }
                        }
                        read_relays_writer.set(Some(read_list));
                        write_relays_writer.set(Some(write_list));
                    }
                    _ => {}
                }
            }
        });
    });

    let values = move || {
        let mut values = Vec::with_capacity(2);

        if let Some(read) = read_relays_reader() {
            values.push(("read", DataValue::List(read)));
        }
        if let Some(write) = write_relays_reader() {
            values.push(("write", DataValue::List(write)));
        }

        values
    };

    view! { <DataCard title="BASIC RELAYS" values=values /> }
}
