// see https://developer.holochain.org/api/0.0.38-alpha14/hdk/ for info on using the hdk library
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk_proc_macros::zome;

#[zome]
mod cogov {
	use hdk::holochain_core_types::{
		entry::Entry,
		dna::entry_types::Sharing,
	};
	use hdk::holochain_json_api::{
		json::JsonString,
		error::JsonError,
	};
	use hdk::holochain_persistence_api::{
		cas::content::Address
	};
	use hdk::prelude::{ValidatingEntryType, ZomeApiResult};

	use std::borrow::Borrow;
	use time::{Timespec, Tm};

	#[init]
	fn init() -> Result<(), ()> {
		Ok(())
	}

	#[validate_agent]
	pub fn validate_agent(validation_data: hdk::EntryValidationData<AgentId>) -> Result<(), ()> {
		Ok(())
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct Collective {
		pub name: String,
		pub created_at_sec: i64,
	}

	impl Collective {
		#[allow(dead_code)]
		fn created_at(&self) -> Tm {
			time::at(Timespec::new(self.created_at_sec, 0))
		}
	}

	impl Default for Collective {
		fn default() -> Self {
			Collective {
				name: "unnamed collective".to_string(),
				created_at_sec: time::now_utc().to_timespec().sec,
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct Ledger {
		name: String,
		created_at_sec: i64,
	}

	impl Ledger {
		#[allow(dead_code)]
		fn created_at(&self) -> Tm {
			time::at(Timespec::new(self.created_at_sec, 0))
		}
	}

	impl Default for Ledger {
		fn default() -> Self {
			Ledger {
				name: "unnamed ledger".to_string(),
				created_at_sec: time::now_utc().to_timespec().sec,
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct Proposal {
		pub name: String,
		pub content: String,
		pub created_at_sec: i64,
	}

	impl Proposal {
		#[allow(dead_code)]
		fn created_at(&self) -> Tm {
			time::at(Timespec::new(self.created_at_sec, 0))
		}
	}

	impl Default for Proposal {
		fn default() -> Self {
			Proposal {
				name: "unnamed proposal".to_string(),
				content: "".to_string(),
				created_at_sec: time::now_utc().to_timespec().sec,
			}
		}
	}

//	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
//	pub struct ProposalCreate {
//		name: String,
//		content: String,
//		created?: SystemTime,
//	}

	#[zome_fn("hc_public")]
	fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
		hdk::get_entry(&address)
	}

	// collective
	#[entry_def]
	fn collective_def() -> ValidatingEntryType {
		entry!(
        name: "collective",
        description: "A cogov collective",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Collective>| {
            Ok(())
        }
    )
	}

	#[zome_fn("hc_public")]
	fn commit_collective(collective: Collective) -> ZomeApiResult<Address> {
		let collective_entry = Entry::App("collective".into(), collective.borrow().into());
		let collective_address = hdk::commit_entry(&collective_entry)?;
		create_collective_ledger(&collective, &collective_address)?;
		Ok(collective_address)
	}

	// ledger
	#[entry_def]
	fn ledger_def() -> ValidatingEntryType {
		entry!(
        name: "ledger",
        description: "A cogov collective ledger",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Ledger>| {
            Ok(())
        }
    )
	}

	fn create_collective_ledger(collective: &Collective, collective_address: &Address) -> ZomeApiResult<Address> {
		let ledger_name = format!("Primary Ledger for {}", collective.name).to_string();
		let ledger = Ledger {
			name: ledger_name,
			..Default::default()
		};
		let ledger_address = commit_ledger(ledger)?;
		hdk::link_entries(
			&collective_address,
			&ledger_address,
			"collective_leger",
			"ledger_primary",
		)
	}

	fn commit_ledger(ledger: Ledger) -> ZomeApiResult<Address> {
		let ledger_entry = Entry::App("ledger".into(), ledger.into());
		let ledger_address = hdk::commit_entry(&ledger_entry)?;
		Ok(ledger_address)
	}

	#[entry_def]
	fn proposal_def() -> ValidatingEntryType {
		entry!(
			name: "proposal",
			description: "A pro",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Proposal>| {
				Ok(())
			}
		)
	}

	#[zome_fn("hc_public")]
	fn commit_proposal(proposal: Proposal) -> ZomeApiResult<Address> {
		let proposal_entry = Entry::App("proposal".into(), proposal.into());
		let proposal_address = hdk::commit_entry(&proposal_entry)?;
		Ok(proposal_address)
	}
}
