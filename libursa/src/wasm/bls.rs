use wasm_bindgen::prelude::*;

use super::convert_from_js;
use bls;

#[wasm_bindgen]
pub struct Generator(bls::Generator);

#[wasm_bindgen]
impl Generator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Generator, JsValue> {
        Ok(Generator(maperr!(bls::Generator::new())))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Generator, JsValue> {
        Ok(Generator(maperr!(bls::Generator::from_bytes(bytes))))
    }
}

#[wasm_bindgen]
pub struct SignKey(bls::SignKey);

#[wasm_bindgen]
impl SignKey {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignKey, JsValue> {
        Ok(SignKey(maperr!(bls::SignKey::new(None))))
    }

    #[wasm_bindgen(js_name = fromSeed)]
    pub fn from_seed(seed: &[u8]) -> Result<SignKey, JsValue> {
        Ok(SignKey(maperr!(bls::SignKey::new(Some(seed)))))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<SignKey, JsValue> {
        Ok(SignKey(maperr!(bls::SignKey::from_bytes(bytes))))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.as_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct VerKey(bls::VerKey);

#[wasm_bindgen]
impl VerKey {
    #[wasm_bindgen(constructor)]
    pub fn new(generator: Generator, sign_key: SignKey) -> Result<VerKey, JsValue> {
        Ok(VerKey(maperr!(bls::VerKey::new(&generator.0, &sign_key.0))))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<VerKey, JsValue> {
        Ok(VerKey(maperr!(bls::VerKey::from_bytes(bytes))))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.as_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct ProofOfPossession(bls::ProofOfPossession);

#[wasm_bindgen]
impl ProofOfPossession {
    #[wasm_bindgen(constructor)]
    pub fn new(ver_key: &VerKey, sign_key: &SignKey) -> Result<ProofOfPossession, JsValue> {
        Ok(ProofOfPossession(maperr!(bls::ProofOfPossession::new(
            &ver_key.0,
            &sign_key.0
        ))))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<ProofOfPossession, JsValue> {
        Ok(ProofOfPossession(maperr!(
            bls::ProofOfPossession::from_bytes(bytes)
        )))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.as_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct Signature(bls::Signature);

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Signature, JsValue> {
        Ok(Signature(maperr!(bls::Signature::from_bytes(bytes))))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.as_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct MultiSignature(bls::MultiSignature);

#[wasm_bindgen]
impl MultiSignature {
    #[wasm_bindgen(constructor)]
    pub fn new(signatures: Vec<JsValue>) -> Result<MultiSignature, JsValue> {
        let sigs: Vec<bls::Signature> =
            signatures.iter().map(|x| x.into_serde().unwrap()).collect();
        Ok(MultiSignature(bls::MultiSignature::new(
            sigs.iter().collect::<Vec<_>>().as_slice(),
        )?))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<MultiSignature, JsValue> {
        Ok(MultiSignature(maperr!(bls::MultiSignature::from_bytes(
            bytes
        ))))
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.0.as_bytes().to_vec())
    }
}

#[wasm_bindgen]
pub struct Bls(bls::Generator);

#[wasm_bindgen]
impl Bls {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Bls {
        Bls(bls::Generator::new().unwrap())
    }

    //    pub fn keypair() ->

    pub fn sign(message: &[u8], sign_key: &SignKey) -> Result<Signature, JsValue> {
        Ok(Signature(maperr!(bls::Bls::sign(message, &sign_key.0))))
    }

    pub fn verify(
        message: &[u8],
        signature: &Signature,
        ver_key: &VerKey,
        generator: &Generator,
    ) -> Result<bool, JsValue> {
        Ok(maperr!(bls::Bls::verify(
            &signature.0,
            message,
            &ver_key.0,
            &generator.0
        )))
    }

    #[wasm_bindgen(js_name = verifyProofOfPosession)]
    pub fn verify_proof_of_posession(
        pop: &ProofOfPossession,
        ver_key: &VerKey,
        generator: &Generator,
    ) -> Result<bool, JsValue> {
        Ok(maperr!(bls::Bls::verify_proof_of_posession(
            &pop.0,
            &ver_key.0,
            &generator.0
        )))
    }

    #[wasm_bindgen(js_name = verifyMultiSignature)]
    pub fn verify_multi_signature(
        multi_sig: &MultiSignature,
        message: &[u8],
        ver_keys: Vec<JsValue>,
        gen: &Generator,
    ) -> Result<bool, JsValue> {
        let vks: Vec<bls::VerKey> = ver_keys
            .iter()
            .map(|x| convert_from_js(x).unwrap())
            .collect();
        Ok(maperr!(bls::Bls::verify_multi_sig(
            &multi_sig.0,
            message,
            vks.iter().collect::<Vec<_>>().as_slice(),
            &gen.0
        )))
    }
}
