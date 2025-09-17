pub mod ecdsa;
pub mod merkle;

// #[cfg(test)]
// mod tests {
//     use merkle::validate_merkle_tree;

//     use ecdsa::validate_ecdsa_signature;

//     use super::*;

//     #[test]
//     fn test_validate_merkle_tree() {
//         let sample_leaves: Vec<String> = vec![
// 			"HTTP/1.1 200 OK\r\nDate: Sun, 08 Sep 2024 11:32:39 GMT\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: 209\r\nConnection: close\r\nReport-To: {\"group\":\"heroku-nel\",\"max_age\":3600,\"endpoints\":[{\"url\":\"https://nel.heroku.com/reports?ts=1725782677&sid=e11707d5-02a7-43ef-b45e-2cf4d2036f7d&s=dxmGA3CFr3P4tKzR6kQOerpS%2FCNt3RMuKSMoJYDrIz0%3D\"}]}\r\nReporting-Endpoints: heroku-nel=https://nel.heroku.com/reports?ts=1725782677&sid=e11707d5-02a7-43ef-b45e-2cf4d2036f7d&s=dxmGA3CFr3P4tKzR6kQOerpS%2FCNt3RMuKSMoJYDrIz0%3D\r\nNel: {\"report_to\":\"heroku-nel\",\"max_age\":3600,\"success_fraction\":0.005,\"failure_fraction\":0.05,\"response_headers\":[\"Via\"]}\r\nX-Powered-By: Express\r\nX-Ratelimit-Limit: 1000\r\nX-Ratelimit-Remaining: 999\r\nX-Ratelimit-Reset: 1725782707\r\nVary: Origin, Accept-Encoding\r\nAccess-Control-Allow-Credentials: true\r\nCache-Control: max-age=43200\r\nPragma: no-cache\r\nExpires: -1\r\nX-Content-Type-Options: nosniff\r\nEtag: W/\"d1-AdCHAQW37rE37t8vXTeQZeKV7Cg\"\r\nVia: 1.1 vegur\r\nCF-Cache-Status: HIT\r\nAge: 12482\r\nAccept-Ranges: bytes\r\nServer: cloudflare\r\nCF-RAY: 8bfe9e828bc295f9-JNB\r\nalt-svc: h3=\":443\"; ma=86400\r\n\r\n{\n  \"userId\": 10,\n  \"id\": 98,\n  \"title\": \"laboriosam dolor voluptates\",\n  \"body\": \"doloremque ex facilis sit sint culpa\\nsoluta assumenda eligendi non ut eius\\nsequi ducimus vel quasi\\nveritatis est dolores\"\n}\n\nGET https://jsonplaceholder.typicode.com/posts/98 HTTP/1.1\r\nhost: jsonplaceholder.typicode.com\r\naccept: XXX\r\ncache-control: XXXXXXXX\r\nconnection: XXXXX\r\naccept-encoding: XXXXXXXX\r\n\r\n".to_string(),
// 			"2ba160a93050b676d0e4ae0b929f145f8382fe5920852cfc3ef550f230c1526a".to_string()
// 		];
//         let expected_root_hash =
//             "7136b39c952e510735fef9fdb32a47151cc4474b0d718495a71d18ae88787eab".to_string();
//         let public_key = "c4bb0da5d7cc269bca64a55e2149e6dc91dc7157".to_string();
//         let expected_signature =
// 			"07a53a039f4c2f2338d04953ed2c01753f7454b76c38ba86d3058d3cb449e432673069fd5b7ac15916afeced3c0a7a74fe10679f0006f5a93764ef9cbe96c1db1c".to_string();

//         // generate and validate merkle tree root hash
//         let is_merkle_root_valid = validate_merkle_tree(&sample_leaves, &expected_root_hash);

//         // perform an ecdsa signature verification on the tree root and signature
//         let is_signature_valid =
//             validate_ecdsa_signature(&expected_signature, &expected_root_hash, &public_key)
//                 .unwrap();

//         assert!(is_merkle_root_valid, "INVALID MERKLE ROOT");
//         assert!(is_signature_valid, "INVALID ECDSA SIGNATURE");
//     }
// }
