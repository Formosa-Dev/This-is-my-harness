# Profiles

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [`spec/core/profiles.md`](../spec/core/profiles.md) · [`glossary.md`](../spec/glossary.md) · [`spec/STYLE.md`](../spec/STYLE.md)

This directory holds the per-profile refinements of This is my Harness. The **normative semantics of profiles are defined in [`spec/core/profiles.md`](../spec/core/profiles.md)**; this README defines the directory contract and must not duplicate or contradict that document.

---

## 1. Purpose

A profile is a coherent, expected subset of capabilities for a use case that specializes UX, validators and defaults over the same Core. Profiles specialize; they do not fragment. A profile MUST NOT be a separate standard.

The initial profiles are:

- `developer`
- `web-agent`
- `local-hybrid`
- `multi-agent`
- `workflow-automation`

---

## 2. Directory contract

1. Each profile MUST have a subdirectory named `profiles/<profile-id>/`, where `<profile-id>` is the profile identifier in lowercase kebab-case.
2. A profile subdirectory MUST contain a `README.md` that states, for that profile:
   - which Core semantics it specializes;
   - its additional validators;
   - its defaults;
   - its reference implementations;
   - how it is surfaced for discoverability.
3. A profile README MUST reference [`spec/core/profiles.md`](../spec/core/profiles.md) for the normative semantics. It MUST NOT redefine Core fields or Core meanings.
4. A profile subdirectory MAY contain validators, defaults, examples and reference implementations. These are profile-local; they MUST NOT change Core semantics.
5. A profile MUST NOT introduce a new manifest field with Core-level semantics. A profile-local field MUST be namespaced under the profile so that it cannot collide with a Core field.

---

## 3. Relationship to the Core

1. The Core remains the single source of semantic authority. Where a profile and the Core disagree, the Core governs.
2. A profile MAY add requirements; it MUST NOT remove or weaken a Core requirement.
3. A profile MUST be expressed as capabilities and contracts, not as vendor-specific behavior.
4. A profile that needs a new Core-level concept MUST propose a Core change through an RFC rather than forking.

---

## 4. Versioning

1. Profiles version with the spec `apiVersion`. The policy is [`spec/VERSIONING.md`](../spec/VERSIONING.md); it is referenced here, not restated.
2. A profile MAY carry a profile-local version in addition, but that version MUST NOT be confused with the spec version or a package version.
3. How profiles are versioned is an open question (decision record §58). Until resolved, a change to a profile MUST be recorded through an RFC.

---

## 5. Status

Each profile directory is expected to be added as its reference implementation is built. At the current stage the normative semantics are complete; profile-local material is added incrementally and MUST follow this contract.
