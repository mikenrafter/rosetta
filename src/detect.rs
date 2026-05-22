use crate::detectors;

/// A single detection result from a detector
pub struct Detection {
    /// What format was detected (e.g., "JWT Token", "Unix Timestamp")
    pub label: String,
    /// Confidence from 0.0 to 1.0
    pub confidence: f64,
    /// Key-value pairs of decoded information
    pub fields: Vec<(String, String)>,
}

fn compare_confidence(a: f64, b: f64) -> std::cmp::Ordering {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => std::cmp::Ordering::Equal,
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (false, false) => a.total_cmp(&b),
    }
}

/// Run all detectors against the input, returning results sorted by confidence
pub fn run_all(input: &str) -> Vec<Detection> {
    let all_detectors: Vec<fn(&str) -> Option<Detection>> = vec![
        detectors::jwt::detect,
        detectors::base64::detect,
        detectors::unix_timestamp::detect,
        detectors::uuid::detect,
        detectors::cron::detect,
        detectors::ip::detect,
        detectors::cidr::detect,
        detectors::color::detect,
        detectors::semver::detect,
        detectors::http_status::detect,
        detectors::permissions::detect,
        detectors::url::detect,
        detectors::url_encoded::detect,
        detectors::hex::detect,
        detectors::hash::detect,
        detectors::datetime::detect,
        detectors::docker_image::detect,
        detectors::duration::detect,
    ];

    let mut results: Vec<Detection> = all_detectors
        .iter()
        .filter_map(|detector| detector(input))
        .collect();

    results.sort_by(|a, b| compare_confidence(b.confidence, a.confidence));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_handles_non_finite_confidence_without_panic() {
        let mut results = vec![
            Detection {
                label: "low".into(),
                confidence: f64::NAN,
                fields: vec![],
            },
            Detection {
                label: "high".into(),
                confidence: 0.9,
                fields: vec![],
            },
        ];
        results.sort_by(|a, b| compare_confidence(b.confidence, a.confidence));
        assert_eq!(results[0].label, "high");
    }
}
