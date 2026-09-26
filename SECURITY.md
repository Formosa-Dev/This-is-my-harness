# Security Policy

This is my Harness treats every downloadable harness as **untrusted third-party input**, not as inert configuration. This policy covers vulnerability disclosure and the supply-chain surface of the standard and toolchain.

---

## Project status

This project is **pre-alpha**. There is no stable release, no supported version, no production CLI and no official installer. Security guarantees described in the specification are **design requirements**, not shipped behavior. Nothing here should be relied upon in production.

---

## Reporting a vulnerability

**Please do not report security vulnerabilities through public issues, discussions or pull requests.**

Preferred channel:

1. Use **[GitHub private security advisories](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)** on this repository (Security → Report a vulnerability). This creates a private, tracked conversation with the maintainers.

If you cannot use GitHub, contact the maintainers listed in [MAINTAINERS.md](MAINTAINERS.md) through a private channel (Formosa.dev maintainers — direct contact details TBD).

### What to include

- A description of the issue and the component affected (spec, schema, Core, validator, adapter, CI, distribution).
- The spec `apiVersion` and any relevant version or commit.
- Steps to reproduce, a proof of concept, or a minimal fixture.
- The impact you believe it has, and any suggested mitigation.
- Whether you would like to be credited.

### What to expect

- **Acknowledgement** of your report as soon as reasonably possible.
- An assessment of severity and scope, shared with you.
- A coordinated disclosure timeline, agreed with you, once a fix is available.
- Credit in the release notes if you wish, unless you prefer to remain anonymous.

Please give us a reasonable opportunity to address the issue before any public disclosure. We will not pursue legal action against researchers who act in good faith, avoid privacy violations and data destruction, and do not exploit the issue beyond what is needed to demonstrate it.

**Do not test against systems you do not own or have explicit permission to test.** Do not access, modify or exfiltrate data that is not yours.

### Bug bounty

There is **no bug bounty program** at this time.

---

## Supply-chain surface

The toolchain and standard are exposed to the following threats:

| Threat | Description |
| --- | --- |
| Malicious harness | A published harness that abuses declared capabilities to harm the user's machine. |
| Compromised author | A legitimate publisher account used to ship a malicious version. |
| Tampered artifact | An artifact modified in transit or at rest. |
| Compromised updater | A malicious update delivered through the update channel. |
| Dependency compromise | A transitive dependency of the toolchain itself is compromised. |
| Fake compatibility | A harness claims compatibility it does not have (badge-only conformance). |
| License contamination | A dependency or component that imposes incompatible license terms. |

### Controls

- Immutable, content-addressed versions.
- Content digests verified against the manifest.
- Signatures and attestations (Sigstore / Cosign) verifiable **independently of any social application**.
- Schema validation before any use.
- Secret scanning.
- Human review where appropriate.
- Conformance tests rather than declarative badges.
- A precise trust label — never "100% safe". See below.

---

## Security invariants (design requirements)

The following are **MUST** invariants of the standard. They govern every phase of the build and every artifact:

1. **Project scope by default.** User scope requires an exact preview of affected paths plus an explicit warning.
2. **Preview, snapshot and rollback** are mandatory for every mutation of managed files.
3. **Never auto-execute** third-party scripts, hooks, MCP servers or setup commands without explicit approval. **No code execution during Preview.**
4. **Never request or store runtime provider credentials.** Authentication belongs to the provider.
5. **Immutable releases with mandatory hashes.** Verify the signature **before** Apply.
6. **No "100% safe" claims.** Show exactly what was verified.
7. **Never silently overwrite conflicts. Never delete unmanaged files.**
8. **Path-traversal and symlink-escape protection.** Hash verification. Never resolve arbitrary URLs as commands.
9. **Runtime ≠ Model.** Do not hard-code specific model vendors into the Core.
10. The social layer **MUST NOT** define the semantics of the standard.
11. **Do not publish fictitious install commands. Do not declare v1.0** before the Reference Harnesses.

### Trust label

A trust label reports exactly what was verified, and nothing more:

`manifest valid` · `hash verified` · `signature verified` · `author identity verified` · `maintainer reviewed` · `runtime tested`

A label MUST NOT imply a level of assurance that was not actually verified.

---

## Supported versions

There are no supported versions yet. Once releases exist, this section will list the versions that receive security fixes and their support windows.
