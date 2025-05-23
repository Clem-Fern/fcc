use std::str::FromStr;

use crate::{compliance::misc::ComplianceError, config::FlatConfig};

use super::{process_parent_compliance_check, FlatConfigCompliance};

#[test]
fn test_process_parent_compliance_check_ok() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/1_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/1_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 5);
    for r in result {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_present_is_absent() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/2_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/2_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 2);
    for r in result {
        assert!(r.result.is_err());
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBePresentIsAbsent));
        }
    }
}

#[test]
fn test_process_parent_compliance_check_absent_is_present() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/3_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/3_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 3);
    for r in result {
        assert!(r.result.is_err());
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBeAbsentIsPresent(_)));
        }
    }
}

#[test]
fn test_process_parent_compliance_check_absent_ignore_enum_variant() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/4_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/1_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 2);
    for r in result {
        assert!(r.result.is_err());
        if let Err(err) = r.result {
            assert!(matches!(err, ComplianceError::ShouldBeAbsentIsPresent(_)));
        }
    }
}

#[test]
fn test_process_parent_compliance_check_optional() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/5_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/5_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 3);
    for r in result {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_regex_match_all() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/6_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/6_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 4);
    for r in result {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_regex_match_first() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/7_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/6_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 1);
    assert!(result[0].result.is_ok())
}

#[test]
fn test_process_parent_compliance_check_item_can_match_once_only() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 3);
    for r in result {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_item_match_all() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/9_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 4);
    for r in result {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_absent_match_all() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/10_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 4);
    assert!(result[0].result.is_err());
    assert!(result[1].result.is_err());

    for r in result.iter().take(3).skip(2) {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_check_absent_match_first() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/11_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 3);
    assert!(result[0].result.is_err());

    for r in result.iter().take(2).skip(1) {
        assert!(r.result.is_ok())
    }
}

#[test]
fn test_process_parent_compliance_remove_first_already_match_item_when_identical() {
    let policy = FlatConfigCompliance::from_str(include_str!(
        "../../test/process_parent_compliance_check/12_p.txt"
    ))
    .unwrap();
    let config = FlatConfig::from_str(include_str!(
        "../../test/process_parent_compliance_check/8_c.txt"
    ))
    .unwrap();
    let result = process_parent_compliance_check(&policy, &config);

    assert_eq!(result.len(), 2);
    for r in result {
        assert!(r.result.is_ok())
    }
}
