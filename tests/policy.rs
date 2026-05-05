use copper_dev_config_meter::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 89, capacity: 78, latency: 10, risk: 10, weight: 8 };
    assert_eq!(score(signal), 214);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 65, capacity: 99, latency: 26, risk: 13, weight: 12 };
    assert_eq!(score(signal), 145);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 74, capacity: 71, latency: 23, risk: 10, weight: 13 };
    assert_eq!(score(signal), 168);
    assert_eq!(classify(signal), "accept");
}
