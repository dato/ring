// Copyright 2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Functionality shared by operations on public keys (ECDSA verification and
//! ECDH agreement).

use super::ops::*;
use super::verify_affine_point_is_on_the_curve;
use untrusted;

/// Parses a public key encoded in uncompressed form. The key is validated
/// using the ECC Partial Public-Key Validation Routine from [NIST SP 800-56A,
/// revision
/// 2](http://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-56Ar2.pdf)
/// Section 5.6.2.3.3, the NSA's "Suite B Implementer's Guide to NIST
/// SP 800-56A," Appendix B.3, and the NSA's "Suite B Implementer's Guide to
/// FIPS 186-3 (ECDSA)," Appendix A.3.
pub fn parse_uncompressed_point<'a>(ops: &PublicKeyOps,
                                    input: untrusted::Input<'a>)
                                    -> Result<(Elem, Elem), ()> {
    // NIST SP 800-56A Step 1: "Verify that Q is not the point at infinity.
    // This can be done by inspection if the point is entered in the standard
    // affine representation." (We do it by inspection since we only accept
    // the affine representation.)
    let (x, y) = try!(input.read_all((), |input| {
        // The encoding must be 4, which is the encoding for "uncompressed".
        let encoding = try!(input.read_byte());
        if encoding != 4 {
            return Err(());
        }

        // NIST SP 800-56A Step 2: "Verify that xQ and yQ are integers in the
        // interval [0, p-1] in the case that q is an odd prime p[.]"
        let x = try!(ops.elem_parse(input));
        let y = try!(ops.elem_parse(input));
        Ok((x, y))
    }));

    // NIST SP 800-56A Step 3: "If q is an odd prime p, verify that
    // yQ**2 = xQ**3 + axQ + b in GF(p), where the arithmetic is performed
    // modulo p."
    //
    // `verify_affine_point_is_on_the_curve` does that check and also verifies
    // that the point is on the curve.
    try!(verify_affine_point_is_on_the_curve(ops.common, (&x, &y)));

    // NIST SP 800-56A Note: "Since its order is not verified, there is no
    // check that the public key is in the correct EC subgroup."
    //
    // NSA Suite B Implementer's Guide Note: "ECC Full Public-Key Validation
    // includes an additional check to ensure that the point has the correct
    // order. This check is not necessary for curves having prime order (and
    // cofactor h = 1), such as P-256 and P-384. As long as the implementation
    // under testing claims to support only the Suite B subset of the NIST
    // curves, the partial validation routine will be sufficient to satisfy
    // FIPS 140 CAVP testing of both full and partial public key validation
    // capabilities."

    Ok((x, y))
}


#[cfg(test)]
mod tests {
    use file_test;
    use super::*;
    use super::super::ops;
    use untrusted;

    #[test]
    fn parse_uncompressed_point_test() {
         file_test::run("src/ec/suite_b/suite_b_public_key_tests.txt",
                        |section, test_case| {
            assert_eq!(section, "");

            let curve_name = test_case.consume_string("Curve");
            let curve_ops = if curve_name == "P-256" {
                &ops::p256::PUBLIC_KEY_OPS
            } else if curve_name == "P-384" {
                &ops::p384::PUBLIC_KEY_OPS
            } else {
                panic!("Unsupported curve: {}", curve_name);
            };

            let public_key = test_case.consume_bytes("Q");
            let public_key = untrusted::Input::from(&public_key);
            let valid = test_case.consume_string("Result") == "P";

            let result = parse_uncompressed_point(curve_ops, public_key);
            assert_eq!(valid, result.is_ok());

            // TODO: Verify that we when we re-serialize the parsed (x, y), the
            // output is equal to the input.

            Ok(())
        });
    }
}
