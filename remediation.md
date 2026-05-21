# Security Remediation Report: rosetta

## Audit Metadata
- Repository: https://github.com/JoaquinCampo/rosetta
- Audited commit: `6a4724b33bf457fc87d360f0ab2f135ee2328459`
- Audit date: 2026-04-24
- Auditor workflow: staged source review + dependency advisory scan

## Scope and Method
Three-stage review process:
1. Stage 1: docs removed and full-line comments removed.
2. Stage 2: non-doc files restored (comments restored), docs removed.
3. Stage 3: full docs restored.

Checks covered:
- odd network requests and outbound behavior
- strange encodings and parser safety
- test/prod boundary issues
- file and process operations
- comments/docs for misleading guidance or suspicious links
- dependency CVEs/advisories and attack-surface implications

## Executive Summary
No hidden network behavior, no subprocess execution primitives, and no known dependency vulnerabilities were found in the lockfile scan.

Primary hardening opportunities are input-size limits (to prevent local DoS), safer handling around unsafe terminal checks, and removing a potential panic edge in confidence sorting.

## Findings by Severity

### Medium: Potential Local DoS via Full Stdin Buffering
- Risk: large piped data can cause memory pressure and reduced availability.

Evidence:
- Full stdin read: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/src/main.rs#L44

Remediation:
1. Add max input-size cap with explicit error path.
2. Consider streaming detection mode for very large inputs.

### Low: Potential Panic Edge in Sort Comparator
- Risk: panic if a detector ever returns NaN confidence in future changes.

Evidence:
- `partial_cmp(...).unwrap()`: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/src/detect.rs#L42

Remediation:
1. Replace with total ordering (`total_cmp`) or robust fallback comparator.
2. Add tests asserting confidence values are finite.

### Low: Unsafe FFI Terminal Detection
- Risk: small unsafe boundary; maintainability/soundness overhead.

Evidence:
- Unsafe call: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/src/main.rs#L50
- FFI declaration: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/src/main.rs#L53

Remediation:
1. Prefer safe standard APIs where possible.
2. If unsafe remains, isolate it behind minimal wrapper with platform tests.

### Informational: Decoder `unwrap` in Percent-Decoding Path Appears Guarded
- Observation: `from_str_radix(...).unwrap()` is preceded by hex-digit checks and appears safe under current logic.

Evidence:
- Guarded parse call: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/src/detectors/url_encoded.rs#L48

Recommendation:
1. Keep current guard and add regression tests for malformed `%` patterns.

## Stage-by-Stage Results

### Stage 1 (comments removed, docs removed)
- No odd network requests detected.
- No suspicious subprocess or file-write behaviors.
- Encodings/decoders appear purposeful to product scope (JWT/Base64/URL encoding).

### Stage 2 (comments restored, docs removed)
- Comments generally matched implementation behavior.
- No suspicious links or hidden behavior directives in code comments.

### Stage 3 (docs restored)
- No malicious or deceptive links found in README.
- Documentation appears aligned with runtime features.

References:
- Install link: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/README.md#L25
- Supported URL example: https://github.com/JoaquinCampo/rosetta/blob/6a4724b33bf457fc87d360f0ab2f135ee2328459/README.md#L161

## Test-to-Production Boundary
No evidence that test-only modules are leaking into production build paths.

## Dependency CVE/Advisory Review
Scanner: `cargo-audit` via Nix shell (`nix shell nixpkgs#cargo-audit -c cargo-audit audit --file Cargo.lock`)

Result summary:
- Vulnerabilities found: none
- Informational warnings: none reported for current lockfile snapshot

Attack surface interpretation:
- Current dependency risk appears low based on known advisory database at audit time.
- Main risk remains parser/input robustness and operational limits.

## Three Security Breakdowns

### 1. Exploitability-first view
- No obvious direct RCE path; primary abuse path is resource exhaustion with oversized input.

### 2. Supply-chain view
- Lockfile came back clean for known advisories at scan time.

### 3. Operational resilience view
- Hard limits and safe comparator behavior will materially improve robustness.

## Prioritized Remediation Plan
1. Add configurable max input size and clear over-limit behavior.
2. Replace `partial_cmp(...).unwrap()` with non-panicking total order.
3. Isolate or remove unsafe FFI terminal check.
4. Add CI `cargo-audit` plus parser fuzz/regression tests.

## Verification Checklist
- [ ] Oversized stdin fails safely and predictably.
- [ ] Confidence sort cannot panic on invalid float values.
- [ ] Unsafe terminal check is minimized or replaced.
- [ ] `cargo-audit` remains clean in CI.
