# Normative Style Guide

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **Applies to:** every normative artifact under `spec/`, `profiles/`, `extensions/` and `schemas/`.

This guide defines how the normative specification of This is my Harness is written. It exists so that different authors — human or agent — produce text that is consistent, testable and free of vendor bias. A document that violates this guide is not conformant to the project's own process.

---

## 1. Language and audience

1. All normative artifacts MUST be written in **English**.
2. Normative text MUST be understandable by both humans and machines. Prose MUST NOT rely on tone, humor, marketing language or cultural context to convey a requirement.
3. A statement intended to be enforced by a validator MUST be written as a requirement (see §3), not as advice, a note or an example.

---

## 2. RFC 2119 keywords

The following keywords are used **only** in their uppercase form and **only** to express a normative requirement. Lowercase forms of the same words are ordinary prose and carry no normative weight.

| Keyword | Meaning |
| --- | --- |
| **MUST** | An absolute requirement. A conformant implementation that does not satisfy it is non-conformant. |
| **MUST NOT** | An absolute prohibition. |
| **SHOULD** | A strong recommendation. An implementation MAY deviate, but only for a documented reason. |
| **SHOULD NOT** | A strong discouragement. |
| **MAY** | A genuinely optional behavior. |

Rules for use:

1. A requirement MUST contain exactly one normative keyword and exactly one testable obligation. Combining two obligations in one sentence with two keywords is prohibited.
2. A normative keyword MUST NOT appear inside a NON-NORMATIVE example block or inside an explanatory aside.
3. `REQUIRED`, `OPTIONAL` and `RECOMMENDED` MUST NOT be used as normative keywords. They MAY appear only as column labels in tables, where the corresponding normative meaning MUST be stated in the surrounding text using MUST / SHOULD / MAY.
4. Emphasis (bold, italics) MUST NOT be used to create normative force. Only the keyword does that.

---

## 3. Testable requirements

1. Every requirement MUST be objectively testable by a validator, a conformance fixture or a documented inspection procedure.
2. Vague qualifiers MUST NOT be used in requirements. Prohibited examples include "fast", "secure", "robust", "user-friendly", "reasonable", "as appropriate" and "when possible" unless the term is given a precise, testable definition in the same document.
3. A requirement that cannot currently be tested MUST be marked explicitly as **not yet testable** and MUST NOT be presented as a conformance criterion.
4. When a requirement depends on a concrete surface syntax (field names, file names, regular expressions), the surface syntax MUST be either defined in the document itself or deferred to a named document that defines it. The normative semantics MUST NOT depend on a syntax defined only by an example.
5. Where this document set says a value is "open", the term means the value is **deliberately not fixed** in the current `apiVersion` and MUST NOT be treated as normative. Open items MUST reference the open-questions section of the decision record.

---

## 4. Mandatory vocabulary

1. Every normative artifact MUST use the terms defined in [`glossary.md`](glossary.md) with exactly the meanings defined there. Where a term is capitalized in this document set, it refers to the glossary.
2. A document MUST NOT introduce a private synonym for a glossary term. If an existing term does not cover a needed concept, the correct action is to propose a glossary addition, not to invent a local term.
3. The controlled terms — harness, runtime, model, adapter, capability, extension, profile, component, manifest, install protocol, install plan, snapshot, apply, revert, scope, risk class, autonomy level, canonical identifier, conformance, trust label, reference harness — MUST be used precisely. In particular:
   - **runtime** MUST NOT be used to mean a model, and **model** MUST NOT be used to mean a runtime.
   - **conformance** MUST NOT be used to mean a badge or a declaration.
   - **compatibility** MUST NOT be used to mean "tested" unless conformance evidence exists (see [`core/conformance-metadata.md`](core/conformance-metadata.md)).

---

## 5. Vendor neutrality of the Core

1. Normative text in the Core (this directory, `spec/core/`, `spec/manifest/` and `spec/package/`) MUST NOT hardcode a vendor, product or model brand as a requirement.
2. A capability MUST be expressed as a **capability contract** — a named, versioned unit of functionality independent of any concrete implementation — rather than as a named product.
3. Concrete implementations MAY be named only in a block explicitly marked NON-NORMATIVE, and only as illustrative examples. A named implementation MUST NOT be required for conformance.
4. Runtime-specific and product-specific behavior belongs in an adapter or an extension, never in the Core. A Core document that needs runtime-specific behavior in order to be expressed is evidence that the abstraction is wrong.
5. Model brands and decision-model product names MUST NOT appear in normative Core text. The Core describes model capability contracts; a concrete model is one possible implementation of a contract.

---

## 6. Document structure conventions

1. Every normative document MUST begin with an H1 title followed by a status banner that states the current `apiVersion` and the RFC 2119 interpretation. The banner format used across this document set is the example to follow.
2. Every normative section MUST be either **Normative** (default) or explicitly labelled **NON-NORMATIVE**. Examples are NON-NORMATIVE unless a document states otherwise.
3. Cross-references MUST use relative links to the canonical file. A requirement moved to another file MUST leave a pointer, not a duplicate.
4. Versioning semantics MUST be consistent with [`VERSIONING.md`](VERSIONING.md). A document MUST NOT restate versioning rules that belong to `VERSIONING.md`; it MUST reference them.
5. A document MUST NOT include a complete instance of a manifest or a JSON Schema. Semantics are normative here; concrete schemas are defined separately. Conceptual fragments inside a NON-NORMATIVE example are permitted.

---

## 7. Prohibited statements

The following MUST NOT appear in any normative artifact:

1. Marketing or promotional claims, including superiority, "best", "safest" or "revolutionary".
2. Any claim of absolute safety, including "100% safe", "fully secure" or "guaranteed".
3. A claim that an artifact is "compatible" without a defined compatibility level and conformance evidence (see [`core/compatibility.md`](core/compatibility.md)).
4. A declaration that the specification is `v1` or otherwise stable while the project is pre-alpha.
5. A requirement that depends on a fictional command, package identifier or release channel.
6. A reference to the social/registry layer as a source of technical semantics. The social layer is downstream and MUST NOT define semantics.
7. An instruction to execute third-party code outside of a preview/policy/apply flow.

---

## 8. Review checklist

A normative document is ready for review only when every answer below is "yes".

- Is the document in English and using the banner convention?
- Does every requirement contain exactly one RFC 2119 keyword and one testable obligation?
- Is every requirement testable, or explicitly marked not yet testable?
- Are all controlled terms used with their glossary meaning?
- Does the Core text avoid hardcoded vendors and product brands?
- Are examples clearly marked NON-NORMATIVE?
- Are versioning rules referenced rather than duplicated?
- Are open questions marked as open and linked to the decision record?
