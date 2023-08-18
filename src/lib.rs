mod pb;

use substreams::errors::Error;
use substreams::store::{StoreGet, StoreGetArray};
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use crate::pb::addresslookuptables::types::v1::{AddressLookupTables, Resolved};

#[substreams::handlers::map]
pub fn map_address_lookup_table_resolver(block: Block, address_lookup_table_store: StoreGetArray<String>) -> Result<AddressLookupTables, Error> {
    let mut output = AddressLookupTables::default();


    for confirmed_trx in block.transactions_owned().filter(|trx| trx.meta().is_some()) {
        if let Some(trx) = confirmed_trx.transaction {
            let msg = trx.message.unwrap();

            msg.address_table_lookups.into_iter().for_each(|addr| {
                let address_table_lookup_account = bs58::encode(&addr.account_key).into_string();

                let mut accounts = vec![];
                let mut writable_accounts = vec![];
                let mut readable_accounts = vec![];

                match address_lookup_table_store.get_last(format!("table:{address_table_lookup_account}")) {
                    None => panic!("Address Lookup Table Account {} does not exist", address_table_lookup_account),
                    Some(accs) => {
                        addr.writable_indexes.into_iter().for_each(|idx| {
                            writable_accounts.push(accs[idx as usize].clone());
                        });
                        addr.readonly_indexes.into_iter().for_each(|idx| {
                            readable_accounts.push(accs[idx as usize].clone());
                        })
                    }
                }
                accounts.append(&mut writable_accounts);
                accounts.append(&mut readable_accounts);
                output.address_lookup_tables.insert(address_table_lookup_account.clone(), Resolved{ addresses: accounts.clone() });
            });
        }
    }

    Ok(output)
}
