pub mod ic {
    pub const IDENTITY_PATH: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/ic/identity.pem");
    pub const MANAGED_VERIFIER: &str = "uxrrr-q7777-77774-qaaaq-cai";
    pub const ZKTLS_VERIFIER: &str = "uzt4z-lp777-77774-qaabq-cai";
}

pub mod notary {
    pub const PUB_KEY: &str = include_str!("../../assets/notary/notary.pub");
}

pub mod presentation {
    pub const PRESENTATION_32B: &str =
        include_str!("../../assets/presentation/presentation_32b.json");
    pub const PRESENTATION_1KB: &str =
        include_str!("../../assets/presentation/presentation_1kb.json");
}

pub mod receipt {
    pub const RECEIPT_32B: &[u8] = include_bytes!("../../assets/receipt/receipt_32b.bin");
    pub const RECEIPT_1KB: &[u8] = include_bytes!("../../assets/receipt/receipt_1kb.bin");
}
