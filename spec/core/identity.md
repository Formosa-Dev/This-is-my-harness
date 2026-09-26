# Identity, Naming and Namespace

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [versioning](versioning.md) · [distribution](distribution.md)

This document defines how a harness is named, how its identifier is formed, and how namespaces and renames are handled. It answers the identity part of §2.1 and §17, and reserves the open questions recorded in §58.

---

## 1. Concepts

| Term | Definition |
| --- | --- |
| **Owner** | The namespace authority for a harness. In the canonical URL it is the `<owner>` path segment; in a scoped reference it is the `<scope>` after `@`. |
| **Slug** | The harness name within an owner namespace. |
| **Host** | The DNS authority that serves a canonical identifier and resolves it to a manifest, a version and an immutable artifact. |
| **Namespace** | The pair `(host, owner)`. Two harnesses with the same slug under different owners MUST NOT be considered the same harness. |
| **Canonical identifier** | A stable, globally unique URL for a harness, optionally qualified by a version. |
| **Scoped reference** | The short `@owner/slug` form used inside manifests to refer to a harness or component. |
| **Short reference** | The `<owner>/<slug>` form accepted by a user-facing command, resolved against a default host. |

An owner is not a user account, an email address or an organization record. An owner is a **namespace**; how an implementation maps an owner to an identity is outside this specification.

---

## 2. Character grammar

Identifiers are ASCII. Non-ASCII input MUST be rejected, not transliterated. All identifiers are **lowercase** and MUST be normalized to lowercase before any comparison, storage or lookup.

The following regular expressions are normative.

```text
owner  = /^[a-z0-9](?:[a-z0-9-]{0,38}[a-z0-9])?$/
slug   = /^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$/
```

Properties:

- An `owner` is 1 to 40 characters; a `slug` is 1 to 64 characters.
- Both MUST start and end with a lowercase letter or a digit.
- Both MAY contain hyphens (`-`) in the interior.
- Underscores, dots, spaces, uppercase letters and any other character are prohibited.
- A single-character identifier is valid.

### 2.1 Scoped reference

```text
scoped-reference = "@" owner "/" slug
```

Example shape (NON-NORMATIVE): `@owner/slug`.

### 2.2 Canonical identifier

```text
canonical-identifier = "https://" host "/h/" owner "/" slug
versioned-identifier  = canonical-identifier "@" version
```

- The scheme MUST be `https`. Plain `http` MUST be rejected.
- `host` MUST be a valid DNS name. The choice of host is deployment-specific; the standard does not fix one host.
- `version` MUST be either a SemVer package version (see [`versioning.md`](versioning.md)) or a **channel alias** (for example a moving pointer such as `latest`). A channel alias is mutable by definition and therefore MUST be resolved to an immutable digest before any Apply (see [`distribution.md`](distribution.md)).

The canonical identifier is a URL and MUST resolve reproducibly to exactly one artifact by digest once a version is selected. Two different artifacts MUST NOT share one versioned identifier.

### 2.3 Short reference

```text
short-reference = owner "/" slug
```

A user-facing command MAY accept a short reference and resolve it against a configured default host. A short reference carries no host, so it MUST NOT be used as a durable identifier inside a published manifest; manifests MUST use either a scoped reference resolved against the same host or a canonical identifier.

---

## 3. Normalization and comparison

1. Before comparing, storing or looking up any identifier, an implementation MUST normalize it to lowercase.
2. Comparison of identifiers MUST be exact and case-insensitive only because of rule 1. Two identifiers that differ after normalization MUST be treated as different.
3. Unicode normalization MUST NOT be applied to produce a different ASCII identifier. A character outside the grammar MUST be rejected.
4. A trailing slash, a query string or a fragment MUST NOT be part of a canonical identifier.

---

## 4. Uniqueness

Within a host, the following are unique:

| Scope of uniqueness | Key |
| --- | --- |
| Owner namespace | `owner` |
| Harness | `(owner, slug)` |
| Published version | `(owner, slug, version)` |
| Immutable artifact | `digest` |

1. A host MUST NOT allow two distinct harnesses to share the same `(owner, slug)`.
2. A published `(owner, slug, version)` MUST refer to exactly one artifact digest and MUST be immutable (see [`distribution.md`](distribution.md)).
3. A digest MUST always resolve to the same bytes, independent of any social or registry layer.

---

## 5. Namespaces and `@scope`

1. A namespace is identified by an owner. The `@` prefix in `@owner/slug` exists only to disambiguate a scoped reference from a relative path; the owner grammar itself does not include `@`.
2. An owner namespace MAY contain multiple harnesses and reusable components.
3. A scoped reference MUST resolve against the same host as the manifest that contains it, unless it is written as a canonical identifier.
4. Reserved owner names MAY be defined by governance. A reserved owner name MUST NOT be assignable to a third party, and reserved status MUST be documented.
5. An owner namespace MUST NOT be transferred in a way that re-points an existing published `(owner, slug, version)` to different bytes.

---

## 6. Rename rules

1. A published identifier is **stable**. An owner or slug MUST NOT be silently renamed.
2. A rename MUST produce a **new** identifier. The old identifier MUST NOT be reassigned to a different harness.
3. When a rename is performed, the old identifier SHOULD remain resolvable as a **permanent redirect** or an explicit tombstone that states the new identifier. The redirect MUST NOT serve different content than the new identifier for the same version.
4. A version that has been published under the old identifier MUST remain retrievable by its digest regardless of the rename.
5. Renaming an owner MUST NOT change the identity of any versioned artifact; digests are rename-invariant.
6. A consumer that follows a redirect MUST report the effective canonical identifier it resolved, so that the resolution is explicit and not silent.

---

## 7. Examples

### 7.1 VALID

| Value | Kind | Why valid |
| --- | --- | --- |
| `owner/slug` | short reference | lowercase, interior hyphen optional, both segments conform |
| `@owner/slug` | scoped reference | `@` + owner + `/` + slug |
| `a/b` | short reference | single-character owner and slug are valid |
| `formosa-dev/base-coding` | short reference | interior hyphen, no leading/trailing hyphen |
| `https://example.org/h/owner/slug` | canonical identifier | `https`, valid host, `/h/` path shape |
| `https://example.org/h/owner/slug@1.2.0` | versioned identifier | SemVer qualifier |
| `https://example.org/h/owner/slug@latest` | versioned identifier | channel alias, mutable, resolved to a digest before Apply |
| `owner-1/my-slug-2` | short reference | digits and interior hyphens |

### 7.2 INVALID

| Value | Why invalid |
| --- | --- |
| `Owner/slug` | uppercase in owner |
| `owner/Slug` | uppercase in slug |
| `-owner/slug` | leading hyphen |
| `owner-/slug` | trailing hyphen |
| `owner/slug-` | trailing hyphen in slug |
| `owner_1/slug` | underscore prohibited |
| `owner/slug.md` | dot prohibited |
| `owner/sl ug` | space prohibited |
| `owner/` | empty slug |
| `/slug` | empty owner |
| `owner` | missing slug |
| `https://example.org/h/Owner/slug` | uppercase in canonical identifier |
| `http://example.org/h/owner/slug` | non-`https` scheme |
| `https://example.org/h/owner/slug/` | trailing slash |
| `@owner` | scoped reference missing slug |
| a 65-character slug | exceeds slug length bound |
| a 41-character owner | exceeds owner length bound |

---

## 8. Open items

The following are deliberately not fixed in `v1alpha1` and MUST be treated as open until resolved through an RFC:

1. The definitive namespace and canonical-identifier shape (decision record §58).
2. The authoritative host(s), reserved-owner policy and the mapping between an owner namespace and a publishing identity.

These open items do not suspend the constraints already stated above; an implementation MUST still enforce the grammar, uniqueness and rename rules.
