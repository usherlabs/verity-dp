(function() {
    var implementors = Object.fromEntries([["verity_ic",[["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"verity_ic/crypto/config/enum.Environment.html\" title=\"enum verity_ic::crypto::config::Environment\">Environment</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"verity_ic/crypto/ecdsa/enum.EcdsaKeyIds.html\" title=\"enum verity_ic::crypto::ecdsa::EcdsaKeyIds\">EcdsaKeyIds</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"verity_ic/remittance/types/enum.Action.html\" title=\"enum verity_ic::remittance::types::Action\">Action</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"verity_ic/remittance/types/enum.Chain.html\" title=\"enum verity_ic::remittance::types::Chain\">Chain</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"verity_ic/verify/types/enum.ProofResponse.html\" title=\"enum verity_ic::verify::types::ProofResponse\">ProofResponse</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/crypto/config/struct.Config.html\" title=\"struct verity_ic::crypto::config::Config\">Config</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/crypto/ecdsa/struct.ECDSAPublicKeyReply.html\" title=\"struct verity_ic::crypto::ecdsa::ECDSAPublicKeyReply\">ECDSAPublicKeyReply</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/crypto/ecdsa/struct.SignWithECDSAReply.html\" title=\"struct verity_ic::crypto::ecdsa::SignWithECDSAReply\">SignWithECDSAReply</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/external_router/struct.Account.html\" title=\"struct verity_ic::remittance::external_router::Account\">Account</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.Account.html\" title=\"struct verity_ic::remittance::types::Account\">Account</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.DataModel.html\" title=\"struct verity_ic::remittance::types::DataModel\">DataModel</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.Event.html\" title=\"struct verity_ic::remittance::types::Event\">Event</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.RemittanceReciept.html\" title=\"struct verity_ic::remittance::types::RemittanceReciept\">RemittanceReciept</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.RemittanceReply.html\" title=\"struct verity_ic::remittance::types::RemittanceReply\">RemittanceReply</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.RemittanceSubscriber.html\" title=\"struct verity_ic::remittance::types::RemittanceSubscriber\">RemittanceSubscriber</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.Subscriber.html\" title=\"struct verity_ic::remittance::types::Subscriber\">Subscriber</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.Wallet.html\" title=\"struct verity_ic::remittance::types::Wallet\">Wallet</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/remittance/types/struct.WithheldAccount.html\" title=\"struct verity_ic::remittance::types::WithheldAccount\">WithheldAccount</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.216/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"verity_ic/verify/types/struct.VerificationResponse.html\" title=\"struct verity_ic::verify::types::VerificationResponse\">VerificationResponse</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[6351]}