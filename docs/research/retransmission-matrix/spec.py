#!/usr/bin/env python3
"""Expand/check a DRAFT documentation matrix; not a TMATS implementation/oracle."""
import argparse
import gzip
import hashlib
import io
import itertools
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DOMAINS = {
    'srcc': ['0', '1', 'U'],
    'content': ['B', 'H', 'W', 'D', 'R', 'X'],
    'sha_pair': [a + b for a in 'NVFMDU' for b in 'NVFMDU'],
    'continuity': ['C', 'G', 'O'],
    'frmt_rccver': ['SS', 'SD', 'DS', 'DD'],
    'sealed_state': ['T', 'X', 'A'],
    'separator': ['T', 'S', 'E'],
    'phase': ['I', 'M'],
}


def proposal(v):
    """Draft column-composition rules R0-R5 and overlays in the companion note."""
    s, c, pair, cont, versions, own, sep, phase = v
    a, b = pair
    findings, questions, conflicts = ['retain_all_source_findings'], ['QG'], []
    for side, status in [('sealed', a), ('following', b)]:
        if status in 'FMDU':
            findings.append(side + '_sha_' + {'F': 'mismatch', 'M': 'malformed',
                'D': 'duplicate_F1_indeterminate_refuse_stamp', 'U': 'unavailable_indeterminate'}[status])
    if c == 'B' and a != b:
        conflicts.append('identical_bytes_conflicting_sha_observations')
    if c == 'B' and own == 'X':
        conflicts.append('complete_following_bytes_equal_cut_sealed_text')
    if c == 'H' and ('N' in pair or ((a == 'D') != (b == 'D'))):
        conflicts.append('checksum_value_only_requires_present_matching_item_inventory')
    if own == 'X' and a == 'V':
        conflicts.append('cut_sealed_text_cannot_have_authoritative_verified_status')
    if c == 'X' and b == 'V':
        conflicts.append('cut_following_text_cannot_have_authoritative_verified_status')
    if s == 'U':
        findings.append('following_first_srcc_unreadable')
        questions.append('QS')
    if s == '0' and c in 'HWDR':
        findings.append('unchanged_claim_differs_report_change')
    if s == '1' and c == 'B' and versions == 'SS':
        findings.append('change_claim_but_identical_text')
    if cont == 'G':
        findings.append('sequence_gap_or_other_sequence_retained')
    if cont == 'O':
        findings.append('different_channel_no_implicit_link')
        questions.append('QB')
    if versions != 'SS':
        findings.append('metadata_profile_differs')
        if s == '0':
            findings.append('unchanged_claim_metadata_differs')
        questions.append('QB')
    if versions[0] == 'D':
        findings.append('following_FRMT_XML_unsupported')
        findings.append('ASCII_XML_mixing_requires_session_provenance_check')
    if sep == 'E':
        findings.append('EOF_cross_input_comparison_only')
        questions.append('QE')
    if c in 'HWR':
        questions.append('QC')
    if c == 'X':
        findings.append('following_incomplete_comparison_not_final')
        questions.append('QP')
    # R1: no identity/digest rescue of an already cut or ambiguous candidate.
    outcome = {'T': 'Complete', 'X': 'Incomplete', 'A': 'Ambiguous'}[own]
    rule = {'T': 'R1T', 'X': 'R1X', 'A': 'R1A'}[own]
    basis = {'T': ('time-boundary seal (owner policy)' if phase == 'M' else
                   'initial time seal (INT-045)'), 'S': 'same-channel stop',
             'E': 'end of input (labelled)'}[sep]
    if own != 'T':
        basis += '; retain pre-existing ' + outcome
    # R2: duplicate checksum remains blocking; field ambiguity != settled assembly rule.
    if own == 'T' and a == 'D':
        outcome, rule = 'Ambiguous', 'R2D'
        basis += '; duplicate_G_SHA_assembly_precedence_proposed_QD'
        questions.append('QD')
    guard = sep == 'T' and cont == 'C' and versions == 'SS'
    if own == 'T' and a != 'D' and guard:
        if c == 'B':
            outcome, rule = 'Complete', 'R3B'
            basis += '; identical retransmission'
            findings.append('byte_identical_retransmission_evidence')
            questions.append('QI')
        elif a == 'V':
            outcome, rule = 'Complete', 'R3V'
            basis += '; sealed_G_SHA_verified_classification_only'
            findings.append('possible_premature_time_boundary_seal_both_readings')
        else:
            outcome, rule = 'Ambiguous', 'R3A'
            basis += '; possible premature time-boundary seal; both readings'
            findings.append('possible_premature_time_boundary_seal_both_readings')
    # R4: following classification is comparison evidence, not governing permission.
    if c == 'X':
        following = 'continuation_unresolved_cut' if guard else 'incomplete_unresolved_candidate'
    elif versions != 'SS':
        following = 'change_metadata_with_' + {'B': 'identical_text', 'H': 'sha_value_diff',
                    'W': 'raw_layout_diff', 'D': 'semantic_diff', 'R': 'other_raw_diff'}[c]
    else:
        following = {'B': 'identical_retransmission', 'H': 'change_sha_value_only',
                     'W': 'change_raw_layout_only', 'D': 'change_semantic', 'R': 'change_other_raw_semantics_unsettled'}[c]
    if guard and c != 'B':
        following += '; continuation_reading_retained_no_rejoin'
    if cont == 'O':
        following += '; independent_channel_unlinked'
    if versions[0] == 'D':
        following += '; unsupported_following_format'
    if b == 'D':
        following += '; duplicate_SHA_indeterminate'
    if any(x in pair for x in 'FMDU'):
        questions.append('QH')
    governing = ('eligible_for_review_no_automatic_switch' if outcome == 'Complete'
                 else 'new_adoption_blocked_no_silent_old_fallback')
    if c == 'X' or versions[0] == 'D' or s == 'U':
        governing += '; following_usability_unresolved'
    if conflicts:
        # R0 precedence: explicitly enumerate impossible observations; never prune them.
        outcome = 'Incomplete' if own == 'X' else 'Ambiguous'
        rule, basis = 'R0', 'inconsistent_observations; proposed_conservative_hold'
        following, governing = 'inconsistent_evidence_not_adopted', 'blocked_pending_evidence_correction'
        questions = ['Q0']
        findings.extend(conflicts)
    return dict(rule=rule, sealed_outcome=outcome, sealed_basis=basis,
                following_outcome=following, findings=sorted(set(findings)),
                governing_effect=governing, owner_questions=sorted(set(questions)),
                realizability='inconsistent_observations' if conflicts else 'candidate_fixture_required',
                authority='DRAFT; existing_rules_and_owner_direction; NOT_APPROVED')


def generate():
    buf, classes, count = io.BytesIO(), {}, 0
    with gzip.GzipFile(filename='', fileobj=buf, mode='wb', mtime=0) as z:
        for values in itertools.product(*DOMAINS.values()):
            outcome = proposal(values)
            key = json.dumps(outcome, sort_keys=True, separators=(',', ':'))
            class_id = 'C' + hashlib.sha256(key.encode()).hexdigest()[:16]
            existing = classes.setdefault(class_id, dict(outcome=outcome, members=0, member_ids=[]))
            assert existing['outcome'] == outcome, 'class hash collision'
            existing['members'] += 1
            member = '.'.join(values)
            existing['member_ids'].append(member)
            pair = values[2]
            presence = ('none' if pair == 'NN' else 'following_only' if pair[0] == 'N'
                        else 'sealed_only' if pair[1] == 'N' else 'both')
            row = dict(member=member, inputs=dict(zip(DOMAINS, values)), sha_presence=presence,
                       equivalence_class=class_id, planned_test='retransmission_matrix_' + member.replace('.', '_'),
                       **outcome)
            z.write((json.dumps(row, sort_keys=True, separators=(',', ':')) + '\n').encode())
            count += 1
    index = json.dumps(classes, sort_keys=True, separators=(',', ':')).encode()
    compressed_index = io.BytesIO()
    with gzip.GzipFile(filename='', fileobj=compressed_index, mode='wb', mtime=0) as z:
        z.write(index)
    index = compressed_index.getvalue()
    manifest = dict(status='DRAFT; no approvals', combinations=count, classes=len(classes),
                    domains=DOMAINS, members_sha256=hashlib.sha256(buf.getvalue()).hexdigest(),
                    classes_sha256=hashlib.sha256(index).hexdigest())
    return {'members.jsonl.gz': buf.getvalue(), 'classes.json.gz': index,
            'manifest.json': (json.dumps(manifest, indent=2) + '\n').encode()}, count, len(classes)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument('--write', action='store_true')
    g.add_argument('--check', action='store_true')
    g.add_argument('--expand', action='store_true')
    g.add_argument('--class-id')
    args = p.parse_args()
    if args.expand or args.class_id:
        with gzip.open(ROOT / 'members.jsonl.gz', 'rt') as src:
            for line in src:
                if args.expand or json.loads(line)['equivalence_class'] == args.class_id:
                    print(line, end='')
        return
    files, count, classes = generate()
    for name, data in files.items():
        path = ROOT / name
        if args.write:
            path.write_bytes(data)
        elif not path.exists() or path.read_bytes() != data:
            raise SystemExit('documentation matrix drift: ' + name)
    print(f'{count} combinations; {classes} equivalent outcome classes; documentation only')


if __name__ == '__main__':
    main()
