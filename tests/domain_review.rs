use copper_dev_config_meter::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 61, slack: 40, drag: 14, confidence: 70 };
    assert_eq!(review_score(case), 190);
    assert_eq!(review_lane(case), "ship");
}
