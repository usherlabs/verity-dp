import { createPublicKey } from "crypto";

/**
 * Convert [`publicKey`] into the
 * `Elliptic-Curve-Point-to-Octet-String` encoding described in
 * SEC 1: Elliptic Curve Cryptography (Version 2.0) section 2.3.3 (page 10).
 * <http://www.secg.org/sec1-v2.pdf>
 */
export function to_sec1_bytes(publicKey: string): number[] {
  const spki_der = createPublicKey(publicKey).export({
    format: 'der',
    type: 'spki'
  });

  // For uncompressed P-256 (65 bytes)
  return [0x02].concat(...spki_der.subarray(-64, -32));
}