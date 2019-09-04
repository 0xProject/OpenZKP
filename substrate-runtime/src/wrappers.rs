use parity_codec::{Decode, Encode};
use starkdex;

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PublicKey {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MakerMessage {
    pub vault_a:  u32,
    pub vault_b:  u32,
    pub amount_a: u64,
    pub amount_b: u64,
    pub token_a:  [u8; 24],
    pub token_b:  [u8; 24],
    pub trade_id: u32,
    pub sig:      Signature,
}

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TakerMessage {
    pub maker_message: MakerMessage,
    pub vault_a:       u32,
    pub vault_b:       u32,
}

#[derive(PartialEq, Encode, Default, Clone, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vault {
    pub owner:    PublicKey,
    pub token_id: [u8; 24],
    pub balance:  u64,
}

impl Vault {
    pub fn vault_hash(self) -> [u8; 32] {
        let mut vec_holder = self.token_id.to_vec();
        vec_holder.extend_from_slice(&self.balance.to_be_bytes());
        let mut packed_array = [0_u8; 32];
        packed_array.copy_from_slice(vec_holder.as_slice());

        hash(self.owner.x, packed_array)
    }
}

impl From<PublicKey> for ([u8; 32], [u8; 32]) {
    fn from(key: PublicKey) -> ([u8; 32], [u8; 32]) {
        (key.x, key.y)
    }
}

impl From<([u8; 32], [u8; 32])> for PublicKey {
    fn from(key: ([u8; 32], [u8; 32])) -> Self {
        Self { x: key.0, y: key.1 }
    }
}

impl From<Signature> for ([u8; 32], [u8; 32]) {
    fn from(key: Signature) -> ([u8; 32], [u8; 32]) {
        (key.r, key.s)
    }
}

impl From<([u8; 32], [u8; 32])> for Signature {
    fn from(key: ([u8; 32], [u8; 32])) -> Self {
        Self { r: key.0, s: key.1 }
    }
}

impl From<MakerMessage> for starkdex::wrappers::MakerMessage {
    fn from(message: MakerMessage) -> Self {
        Self {
            vault_a:  message.vault_a,
            vault_b:  message.vault_b,
            amount_a: message.amount_a,
            amount_b: message.amount_b,
            token_a:  message.token_a,
            token_b:  message.token_b,
            trade_id: message.trade_id,
        }
    }
}

pub fn taker_verify(taker_message: TakerMessage, sig: &Signature, public: &PublicKey) -> bool {
    starkdex::wrappers::taker_verify(
        &taker_message.maker_message.into(),
        taker_message.vault_a,
        taker_message.vault_b,
        (&sig.r, &sig.s),
        (&public.x, &public.y),
    )
}

pub fn verify(hash: [u8; 32], sig: &Signature, public: &PublicKey) -> bool {
    starkdex::wrappers::verify(&hash, (&sig.r, &sig.s), (&public.x, &public.y))
}

pub fn maker_verify(message: MakerMessage, sig: &Signature, public: &PublicKey) -> bool {
    starkdex::wrappers::maker_verify(&message.into(), (&sig.r, &sig.s), (&public.x, &public.y))
}

pub fn hash(in_a: [u8; 32], in_b: [u8; 32]) -> [u8; 32] {
    starkdex::wrappers::hash(&in_a, &in_b)
}

#[allow(dead_code)]
pub fn maker_hash(message: &MakerMessage) -> [u8; 32] {
    starkdex::wrappers::maker_hash(&message.clone().into())
}

#[allow(dead_code)]
pub fn taker_hash(message: &TakerMessage) -> [u8; 32] {
    starkdex::wrappers::taker_hash(
        &maker_hash(&message.maker_message),
        message.vault_a,
        message.vault_b,
    )
}
