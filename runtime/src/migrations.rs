use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
use sp_core::ecdsa;
use sp_runtime::impl_opaque_keys;
use sp_std::vec::Vec;

use crate::RuntimeBlockWeights;
use crate::{AccountId, Aura, BeefyId, Session};

pub type Migrations = SessionKeysMigration;

impl_opaque_keys! {
	pub struct SessionKeysOld {
		pub aura: Aura,
	}
}

/// Generates a `BeefyId` from the given `AccountId`. The resulting `BeefyId` is
/// a dummy value and this is a utility function meant to be used when migration
/// session keys.
pub fn dummy_beefy_id_from_account_id(a: AccountId) -> BeefyId {
	let mut id_raw = [0u8; 33];

	// NOTE: AccountId is 32 bytes, whereas BeefyId is 33 bytes.
	id_raw[1..].copy_from_slice(a.as_ref());
	id_raw[0..4].copy_from_slice(b"beef");

	ecdsa::Public(id_raw).into()
}

pub struct SessionKeysMigration;

impl OnRuntimeUpgrade for SessionKeysMigration {
	fn on_runtime_upgrade() -> Weight {
		Session::upgrade_keys::<SessionKeysOld, _>(|id, keys| crate::SessionKeys {
			aura: keys.aura,
			beefy: dummy_beefy_id_from_account_id(id),
		});
		RuntimeBlockWeights::get().max_block
	}
}
