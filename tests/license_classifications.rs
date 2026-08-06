// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! Port of python-ort's `tests/test_license_classifications.py`.
//!
//! `LicenseClassifications` in this port carries a `ponytail:` note (see
//! `src/models/license_classifications.rs`): the `merge`/`licenses_by_category`/`is_categorized`/
//! `category_names`/subscript query-convenience methods from python-ort were deliberately not
//! ported, since they're not part of the data model. So `TestLicensesByCategory`,
//! `TestCategoriesByLicense`, `TestCategoryNames` and `TestMerge` from the python suite have no
//! equivalent here; only `TestInit`, which exercises `Model::validate`, is ported.

mod common;

use std::collections::HashSet;

use modelo::models::ort::license_categorization::LicenseCategorization;
use modelo::models::ort::license_category::LicenseCategory;
use modelo::models::ort::license_classifications::LicenseClassifications;
use modelo::models::Model;

#[test]
fn detect_duplicate_category_names() {
    let classifications = LicenseClassifications {
        categories: vec![
            LicenseCategory {
                name: "Category 1".to_string(),
                description: String::new(),
            },
            LicenseCategory {
                name: "Category 2".to_string(),
                description: "Another category".to_string(),
            },
            LicenseCategory {
                name: "Category 1".to_string(),
                description: "Duplicate; should cause a failure".to_string(),
            },
        ],
        categorizations: Vec::new(),
    };

    let err = classifications
        .validate()
        .expect_err("expected a validation error for duplicate category names");
    assert!(err.to_string().contains("Category 1"));
}

#[test]
fn detect_duplicate_license_ids() {
    let classifications = LicenseClassifications {
        categories: vec![LicenseCategory {
            name: "permissive".to_string(),
            description: String::new(),
        }],
        categorizations: vec![
            LicenseCategorization {
                id: "ASL-1".to_string(),
                categories: HashSet::new(),
            },
            LicenseCategorization {
                id: "ASL-2".to_string(),
                categories: HashSet::new(),
            },
            LicenseCategorization {
                id: "ASL-1".to_string(),
                categories: HashSet::from(["permissive".to_string()]),
            },
        ],
    };

    let err = classifications
        .validate()
        .expect_err("expected a validation error for duplicate license ids");
    assert!(err.to_string().contains("ASL-1"));
}

#[test]
fn detect_licenses_referencing_non_existing_categories() {
    let classifications = LicenseClassifications {
        categories: vec![
            LicenseCategory {
                name: "Category 1".to_string(),
                description: String::new(),
            },
            LicenseCategory {
                name: "Category 2".to_string(),
                description: String::new(),
            },
        ],
        categorizations: vec![
            LicenseCategorization {
                id: "ASL-1".to_string(),
                categories: HashSet::from(["Category 1".to_string()]),
            },
            LicenseCategorization {
                id: "ASL-2".to_string(),
                categories: HashSet::from(["unknownCategory".to_string()]),
            },
            LicenseCategorization {
                id: "BSD".to_string(),
                categories: HashSet::from(["anotherUnknownCategory".to_string()]),
            },
        ],
    };

    let err = classifications
        .validate()
        .expect_err("expected a validation error for licenses referencing non-existing categories");
    let message = err.to_string();
    // This port's `validate` reports the first offending categorization it finds and stops,
    // unlike python-ort which collects and reports all offenders in one error message; only the
    // first invalid reference ("ASL-2" / "unknownCategory") is guaranteed to be mentioned.
    assert!(!message.contains("ASL-1"));
    assert!(message.contains("ASL-2"));
    assert!(message.contains("unknownCategory"));
}

#[test]
fn license_classifications_yml_loads_and_validates() {
    let value = common::load_yaml("license-classifications.yml");
    let classifications: LicenseClassifications =
        serde_yaml::from_value(value).expect("failed to deserialize license-classifications.yml");
    classifications
        .validate()
        .expect("license-classifications.yml should validate cleanly");
    assert!(!classifications.categories.is_empty());
    assert!(!classifications.categorizations.is_empty());
}
