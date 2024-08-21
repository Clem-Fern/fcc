use crate::{compliance::misc::ComplianceError, config::FlatConfig};

use super::{process_parent_compliance_check, FlatConfigCompliance};

#[test]
fn test_process_parent_compliance_check_ok() {
    let policy = FlatConfigCompliance::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/1_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/1_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 5);
    for r in result {
        assert!(matches!(r.result, Ok(_)))
    }
}

#[test]
fn test_process_parent_compliance_check_present_is_absent() {
    let policy = FlatConfigCompliance::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/2_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/2_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 2);
    for r in result {
        assert!(matches!(r.result, Err(_)));
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBePresentIsAbsent));
        }
    }
}

#[test]
fn test_process_parent_compliance_check_absent_is_present() {
    let policy = FlatConfigCompliance::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/3_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/3_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 3);
    for r in result {
        assert!(matches!(r.result, Err(_)));
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBeAbsentIsPresent));
        }
    }
}

#[test]
fn test_process_parent_compliance_check_absent_ignore_enum_variant() {
    let policy = FlatConfigCompliance::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/4_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::new_from_raw(include_str!(
        "../../test/process_parent_compliance_check/1_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 2);
    for r in result {
        assert!(matches!(r.result, Err(_)));
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBeAbsentIsPresent));
        }
    }
}
