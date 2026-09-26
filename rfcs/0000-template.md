# RFC-0000: <Short, descriptive title>

| Field | Value |
| --- | --- |
| **RFC number** | 0000 (replace with the next free number) |
| **Title** | <Short, descriptive title> |
| **Status** | Draft / Discussion / Accepted / Rejected / Withdrawn / Superseded |
| **Authors** | <Name(s) and, optionally, GitHub handle(s)> |
| **Created** | YYYY-MM-DD |
| **Target** | Harness Core Spec / capability extension / adapter contract / install protocol / process |
| **Spec apiVersion** | `thisismyharness.dev/v1alpha1` (or the version this RFC would produce) |
| **Requires** | <Other RFCs, ADRs, or "None"> |

> Copy this file to `rfcs/NNNN-<slug>.md` using the next free number. Fill in every section. Delete the guidance in angle brackets. An RFC MUST document its rationale **and the alternatives it rejected** before it can be accepted.

---

## Summary

<One paragraph. If a reader read only this section, what would they need to know?>

## Motivation

<The problem. Why does the current specification or toolchain fail to address it? Who is affected — harness authors, runtime adapter authors, users, agents? What is the cost of not doing this?>

## Guide-level explanation

<Explain the change as if teaching it to someone who will use it. Introduce new concepts and terms, and show a small, realistic example of a harness manifest or workflow using it. Keep it concrete.>

```yaml
# Example manifest fragment demonstrating the proposal
```

## Reference-level explanation

<The precise, implementable definition. Include:
- the exact shape of any new or changed fields;
- the JSON Schema impact;
- validation and error behavior (typed errors, not silent failures);
- compatibility and capability-loss reporting;
- interaction with existing components and the Install Protocol;
- conformance requirements and how they would be tested.>

## Drawbacks

<Honest costs. Complexity added to the Core, migration burden, ambiguity introduced, ongoing maintenance, security surface.>

## Rationale and alternatives

<Why this design over the others. Then list the alternatives considered, and for each: what it was, why it was rejected, and whether it could be revisited.>

| Alternative | Description | Why rejected |
| --- | --- | --- |
| <Option B> | <What it is> | <Reason> |

## Prior art

<Existing standards, implementations or discussions this builds on or deliberately diverges from. Cite them. Note any open standard this MUST compose rather than reinvent.>

## Security and supply-chain considerations

<Does this affect trust, signatures, permissions, risk class, autonomy, path safety, or execution? If it can cause code execution, says so explicitly. If it expands the attack surface, describe the mitigation.>

## Core vs extension

<Argue whether this belongs in the Core or in an extension, against the criteria in GOVERNANCE.md. If Core: explain why it is required by nearly every harness and free of vendor lock-in. If extension: name the extension.>

## Backwards compatibility

<Is this compatible with `v1alpha1`? Does it require a version bump? What is the migration path? Is the migration reversible? Confirm that no undocumented capability loss is introduced.>

## Unresolved questions

- <Question 1>
- <Question 2>

## Future possibilities

<Related work that is explicitly out of scope for this RFC but naturally follows.>

## References

- <Links to specification sections, related RFCs, ADRs, Engram observations, prior art.>
