# Exported classes

## Entry points

The root of a parsed ORT result document is
[`OrtResult`](crate::models::ort::ort_result::OrtResult), which bundles the
[`Repository`](crate::models::ort::repository::Repository) that was scanned together with
the optional `AnalyzerRun`, `ScannerRun`, `AdvisorRun` and `EvaluatorRun`.

```
use modelo::models::Model;
use modelo::models::ort::ort_result::OrtResult;
use modelo::models::ort::repository::Repository;
use std::collections::HashMap;

let result = OrtResult {
    repository: Repository::default(),
    analyzer: None,
    scanner: None,
    advisor: None,
    evaluator: None,
    labels: HashMap::new(),
};
result.validate().expect("a default OrtResult is valid");
println!("{result}");
```

## Identifiers

[`Identifier`](crate::models::ort::identifier::Identifier) is the package coordinate type
used throughout ORT results (`type:namespace:name:version`). It implements
`FromStr`/`Display` and (de)serializes to/from that single colon-delimited string form.

```
use modelo::models::ort::identifier::Identifier;

let id: Identifier = "Maven:org.example:artifact:1.0".parse().unwrap();
assert_eq!(id.name, "artifact");
assert_eq!(id.to_string(), "Maven:org.example:artifact:1.0");

let json = serde_json::to_string(&id).unwrap();
assert_eq!(json, "\"Maven:org.example:artifact:1.0\"");
```

## Advisor & vulnerabilities

[`AdvisorResult`](crate::models::ort::advisor_result::AdvisorResult) wraps the
[`AdvisorDetails`](crate::models::ort::advisor_details::AdvisorDetails) of the advisor that
produced it, an [`AdvisorSummary`](crate::models::ort::advisor_summary::AdvisorSummary) of
the run, and any [`Vulnerability`](crate::models::ort::vulnerability::Vulnerability) entries
found (each backed by zero or more
[`VulnerabilityReference`](crate::models::ort::vulnerability_reference::VulnerabilityReference)s
with severity/scoring metadata).

```
use modelo::models::Model;
use modelo::models::ort::advisor_details::AdvisorDetails;
use modelo::models::ort::advisor_result::AdvisorResult;
use modelo::models::ort::advisor_summary::AdvisorSummary;
use modelo::models::ort::vulnerability::Vulnerability;

let yaml = r#"
advisor:
  name: VulnerableCode
summary:
  start_time: "2026-01-01T00:00:00Z"
  end_time: "2026-01-01T00:01:00Z"
vulnerabilities:
  - id: CVE-2026-0001
    references:
      - url: "https://example.com/CVE-2026-0001"
        scoring_system: CVSS3
        severity: HIGH
        score: 7.5
"#;

let result: AdvisorResult = serde_yaml::from_str(yaml).unwrap();
result.validate().expect("valid AdvisorResult");
assert_eq!(result.advisor.name, "VulnerableCode");
assert_eq!(result.vulnerabilities.len(), 1);
assert_eq!(result.vulnerabilities[0].id, "CVE-2026-0001");

// Types can also be built directly instead of parsed from YAML.
let advisor = AdvisorDetails {
    name: "OSV".to_string(),
    capabilities: None,
};
let summary = AdvisorSummary {
    start_time: "2026-01-01T00:00:00Z".to_string(),
    end_time: "2026-01-01T00:01:00Z".to_string(),
    issues: Vec::new(),
};
let vulnerability = Vulnerability {
    id: "CVE-2026-0002".to_string(),
    summary: None,
    description: None,
    references: Vec::new(),
    first_fixed_versions: Default::default(),
};
vulnerability.validate().expect("id is non-empty");
let built = AdvisorResult {
    advisor,
    summary,
    vulnerabilities: vec![vulnerability],
    defects: Vec::new(),
};
assert_eq!(built.vulnerabilities[0].id, "CVE-2026-0002");
```

## Provenance & VCS

[`VcsInfo`](crate::models::ort::vcs_info::VcsInfo) describes where a package's source lives
(VCS type, URL, revision, path).

```
use modelo::models::ort::vcs_info::VcsInfo;

let vcs = VcsInfo {
    url: "https://github.com/oss-review-toolkit/ort.git".to_string(),
    revision: "main".to_string(),
    ..Default::default()
};
assert_eq!(vcs.revision, "main");
```

See the [`ort`](crate::models::ort) module for the full list of exported model types, grouped
by ORT run stage (analyzer, scanner, advisor, evaluator) and supporting
configuration/repository types.
