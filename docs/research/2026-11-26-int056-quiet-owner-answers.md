# INT-056: owner clarification of Quiet suppression

Date: 2026-11-26. Owner: Joey. Owner-signed clarification under ADR-0033;
independent AI recheck of amended INT-056 is pending. This supersedes the
initial warning-only emission wording, which remains in earlier historical
records. It does not change Strict/Warn or INT-057–060's behaviour. The latter
entries already have independent CONCUR at PR #52 head
`8ec64115fae70f5e7dc70c744a6964aeebd749bd`, relayed by the owner; the
register now records that concurrence without waiving development gates.

## Owner answers (verbatim)

> (1) A: Quiet suppresses EXACTLY the bad-data findings Warn would attach, of ANY severity. They stay readable from the target's field results and source diagnostics.
> (2) A: every Quiet result carries a marker 'suppressed: N bad-data findings' (N may be 0), so a Quiet result is never confused with clean data.

## Applied contract

Use Warn's bad-data finding set as the exact comparison oracle. Quiet
suppresses those same findings from its assumption result, regardless of
severity. It does not suppress unrelated findings just because they are
warnings, and it does not mutate severity or erase diagnostics. The target's
field results AND source diagnostics keep the suppressed findings readable.
Caller-assumption label, selected level, states/reasons and setup limits stay
visible. A selected target is never upgraded to valid or evidenced by Quiet.

Every Quiet result carries `suppressed: N bad-data findings`, including zero.
N counts the findings suppressed from Warn's corresponding bad-data set,
not merely warning-severity findings, bad fields or other unrelated findings.
Dependent result presentation retains the marker and recorded level. No
marker/suppression converts an eligibility/contradiction refusal into an
accepted choice; the other owner policies remain independent guards.

## Planned checks and authority

Update all three Quiet bad-data fixtures (Invalid/duplicate/unavailable,
with separate ID/format arms). Add a zero-count case with usable target and
no bad-data findings, a positive-count case with information/warning/error
bad-data findings plus unrelated findings, and a dependent-result marker case.
Compare Warn/Quiet on the same source evidence and choice so membership and
counts come from the independently specified bad-data set, not the emitter.

Chapter 9's field definitions remain the baseline sources cited in INT-056
(Table 9-4 TK1 p. 9-48 and time format p. 9-72, 106-24R1). Quiet emission,
its marker and count semantics are explicitly OWNER policy, not a new TMATS
attribute, consumer decoding rule or standards default. No public Rust or
JSON field name is frozen; the required marker text/meaning is explicit.

Local checks audit verbatim agreement, fixture names/links and positive/zero
count expectations, regenerate/check trace and check whitespace. GitHub CI
supplies heavy checks. No Rust/runtime tests, full development analysis or
implementation approval is claimed; source freshness and executable-test
gates remain after independent review.
