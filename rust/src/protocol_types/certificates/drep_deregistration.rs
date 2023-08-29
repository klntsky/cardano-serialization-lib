use crate::*;

#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
)]
#[wasm_bindgen]
pub struct DrepDeregistration {
    pub(crate) voting_credential: StakeCredential,
    pub(crate) coin: Coin,
}

impl_to_from!(DrepDeregistration);

#[wasm_bindgen]
impl DrepDeregistration {
    pub fn voting_credential(&self) -> StakeCredential {
        self.voting_credential.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(voting_credential: &StakeCredential, coin: &Coin) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}
