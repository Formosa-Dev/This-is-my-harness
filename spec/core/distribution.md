# Distribution Metadata

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [versioning](versioning.md) · [identity](identity.md) · [compatibility](compatibility.md) · [manifest](../manifest/README.md)

This document defines the distribution metadata of a harness: how an artifact is addressed, identified, verified and attributed. It answers §35 and §2.1, and records the distribution-related open question from §58.

---

## 1. Transport

1. Harness artifacts MUST be distributed through a **content-addressed** transport. The standard adopts an existing content-addressed artifact ecosystem rather than inventing a bespoke binary transport.
2. The recognized transport is OCI/ORAS semantics: an artifact is pushed to an OCI registry and pulled by reference, with content digests and referrers for signatures and attestations.
3. The transport is a distribution detail. A consumer MUST be able to verify integrity and signature **independently** of any social or registry application.
4. A public registry MAY provide discovery and metadata; the versioned package itself MUST be distributed as an immutable artifact, not as an arbitrary upload.

---

## 2. Distribution reference

1. A distribution reference MUST identify the registry host, the repository path and the version.
2. A distribution reference MUST resolve to **exactly one digest**. A tag MUST NOT resolve to more than one digest at a time.
3. The repository path SHOULD encode the owner and slug so that the canonical identifier and the distribution reference are consistent. The exact path layout is deployment-specific and MUST NOT be assumed by consumers beyond the semantic mapping.
4. An implementation MUST NOT require a specific registry host for conformance. A conformant artifact MAY be published to any compatible registry.

### 2.1 Conceptual reference shape

The following is a NON-NORMATIVE illustration of the mapping, where `<registry>` is any compatible registry host:

```text
<registry>/<org>/harness/<owner>/<slug>:<version>
```

1. The `<version>` maps to the version qualifier of the canonical identifier (see [`identity.md`](identity.md), §2.2).
2. Consumers MUST resolve the reference to a digest before Apply; resolving to a tag alone is insufficient.

---

## 3. `artifactType` and the media type — OPEN

1. The distribution metadata MUST declare an `artifactType` so that a registry and a consumer can distinguish a harness artifact from a container image or an unrelated artifact.
2. **The definitive artifact media type is OPEN (decision record §58, open question "definitive OCI media type").** It MUST NOT be fixed at `v1alpha1` and MUST NOT be treated as normative.
3. Any media-type string that appears in the source documents or in an example is a **conceptual placeholder**. It MUST NOT be depended upon for interoperability by a conformant implementation at this stage.
4. The semantics that ARE fixed here: the artifact type MUST be stable per artifact kind, MUST be registered before release, and MUST NOT silently change meaning once published.

### 3.1 Non-normative placeholder

The decision record states a conceptual placeholder media type. It is reproduced below **only** to show the shape and is explicitly NON-NORMATIVE and OPEN:

```text
application/vnd.<project>.bundle.<version>
```

No implementation MUST match this string. The placeholder MUST NOT be cited as a settled identifier.

---

## 4. Digests

1. Every published artifact MUST be identified by a content digest.
2. A digest MUST be computed over the exact artifact bytes.
3. A digest MUST always resolve to the same bytes. Immutability by digest is a Core invariant (see [`versioning.md`](versioning.md), §6).
4. The Install Plan MUST record the resolved digest of every artifact it will apply.
5. An artifact whose bytes do not match the expected digest MUST be rejected and MUST NOT be applied.

---

## 5. Signatures and attestations

1. Signatures and attestations MUST be attached as referrers to the artifact, or conveyed by an equivalent content-addressed link, so that they are discoverable from the digest.
2. A verifier MUST be able to verify a signature or attestation without access to any social application.
3. Before Apply, the Core MUST attempt verification of any signature or attestation available for the resolved digest.
4. The result of verification MUST be reported precisely as a **trust label**. A trust label MUST state exactly which checks passed, drawing from at least: manifest valid, hash verified, signature verified, author identity verified, maintainer reviewed, runtime tested.
5. A trust label MUST NOT imply more assurance than was established, and MUST NOT assert absolute safety.

---

## 6. Source repository and provenance

1. Distribution metadata MAY declare the source repository of the artifact.
2. Provenance metadata MUST NOT substitute for signature verification. A stated source is a claim, not evidence.
3. Where an attestation describes how the artifact was built, it SHOULD be published as an attestation referrer so it is verifiable from the digest.

---

## 7. License

1. The license of a package is declared once, in the manifest `metadata.license` (see [`../manifest/README.md`](../manifest/README.md), §3).
2. Distribution metadata MAY restate provenance or licensing information for a registry listing, but it MUST NOT contradict the declared license.
3. A license MUST NOT be inferred from the absence of a declaration. The absence of a declared license MUST be reported as unknown, not as permissive.

---

## 8. Immutability and withdrawal

1. A published version MUST NOT be re-published with different bytes. A correction is a new version (see [`versioning.md`](versioning.md), §6).
2. Withdrawing a version MUST NOT delete the digest from history in a way that makes an existing lock unresolvable without an explicit, reported error.
3. An artifact that fails validation MUST NOT be published as a conformant harness.

---

## 9. Summary

- Transport is content-addressed OCI/ORAS; no bespoke binary transport.
- References resolve to a digest before Apply; tags move, digests do not.
- `artifactType` is required by semantics; the media type is **OPEN** and non-normative.
- Signatures and attestations are referrers, verifiable independently of any social layer.
- Trust labels report exactly what passed; no absolute safety claim.
- License is declared once in the manifest; absence is unknown, not permissive.
