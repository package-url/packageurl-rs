macro_rules! spec_tests {
    ($name:ident, $desc:expr) => {
        mod $name {

            use super::testcase::SpecTestCase;
            use packageurl::PackageUrl;
            use std::str::FromStr;

            lazy_static! {
                static ref TEST_CASE: SpecTestCase<'static> = SpecTestCase::new($desc);
            }

            #[test]
            fn purl_to_components() {
                if let Ok(purl) = PackageUrl::from_str(&TEST_CASE.purl) {
                    assert!(!TEST_CASE.is_invalid);
                    assert_eq!(TEST_CASE.ty, purl.ty);
                    assert_eq!(TEST_CASE.name, purl.name);
                    assert_eq!(TEST_CASE.namespace, purl.namespace);
                    assert_eq!(TEST_CASE.version, purl.version);
                    assert_eq!(TEST_CASE.subpath, purl.subpath);
                    assert_eq!(TEST_CASE.qualifiers, purl.qualifiers);
                } else {
                    assert!(TEST_CASE.is_invalid);
                }
            }

            #[test]
            fn components_to_canonical() {
                if TEST_CASE.is_invalid {
                    return;
                }

                let mut purl = PackageUrl::new(TEST_CASE.ty.as_ref(), TEST_CASE.name.as_ref());

                if let Some(ref ns) = TEST_CASE.namespace {
                    purl.with_namespace(ns.as_ref());
                }

                if let Some(ref v) = TEST_CASE.version {
                    purl.with_version(v.as_ref());
                }

                if let Some(ref sp) = TEST_CASE.subpath {
                    purl.with_subpath(sp.as_ref());
                }

                for (k, v) in TEST_CASE.qualifiers.iter() {
                    purl.add_qualifier(k.as_ref(), v.as_ref());
                }

                assert_eq!(TEST_CASE.canonical_purl, purl.to_string());
            }

            #[test]
            fn canonical_to_canonical() {
                if TEST_CASE.is_invalid {
                    return;
                }

                let purl = PackageUrl::from_str(&TEST_CASE.canonical_purl).unwrap();
                assert_eq!(TEST_CASE.canonical_purl, purl.to_string());
            }

            #[test]
            fn purl_to_canonical() {
                if TEST_CASE.is_invalid {
                    return;
                }
                let purl = PackageUrl::from_str(&TEST_CASE.purl).unwrap();
                assert_eq!(TEST_CASE.canonical_purl, purl.to_string())
            }

        }
    };
}
