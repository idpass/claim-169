use ciborium::Value;
use claim169_core::CwtMeta;

/// Helper to create claim 169 CBOR map
#[allow(dead_code)]
pub fn create_claim169_map(fields: Vec<(i64, Value)>) -> Value {
    Value::Map(
        fields
            .into_iter()
            .map(|(k, v)| (Value::Integer(k.into()), v))
            .collect(),
    )
}

/// Helper to encode CWT
#[allow(dead_code)]
pub fn encode_cwt(meta: &CwtMeta, claim_169: &Value) -> Vec<u8> {
    claim169_core::pipeline::cwt::encode(meta, claim_169)
}

/// Helper to encode a CBOR value to bytes
#[allow(dead_code)]
pub fn encode_cbor(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    ciborium::into_writer(value, &mut out).expect("CBOR encoding should not fail");
    out
}

/// Helper to build Enc_structure AAD for COSE_Encrypt0
#[allow(dead_code)]
pub fn build_encrypt0_aad(protected_bytes: &[u8]) -> Vec<u8> {
    claim169_core::build_encrypt0_aad(protected_bytes).expect("CBOR encoding should not fail")
}
