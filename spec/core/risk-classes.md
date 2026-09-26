# Risk Classes

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [permissions](permissions.md) · [component types](../package/component-types.md)

This document defines the risk classes A–D. It answers §49. A risk class states **what an artifact can do**, not that it is safe, trusted or verified.

---

## 1. The four classes

| Class | Name | Definition |
| --- | --- | --- |
| **A** | **Passive** | Instructions and configuration with no execution. |
| **B** | **Tooling** | MCP servers, plugins and integrations that extend capability, including network tool calls. |
| **C** | **Executable** | Hooks, scripts and commands that execute code. |
| **D** | **Privileged** | User scope, administrative or elevated access, secrets, or broad system access. |

A risk class MUST be assigned to every component whose behavior exceeds Passive, and to every declared permission. A Passive harness MAY omit explicit class assignment because Passive is the default.

---

## 2. Operations each class declares

The following operations define the floor for each class. A component MUST be assigned the highest class among the operations it can perform.

### 2.1 Class A — Passive

Declares operations that:

1. Write declared instruction, configuration or content files within **project scope**.
2. Do not execute any third-party code.
3. Do not open network connections as part of their declared behavior.
4. Do not read or write secrets.

### 2.2 Class B — Tooling

Declares operations that:

1. Register or invoke tools, plugins, integrations or MCP servers.
2. Make network calls to declared endpoints as part of tool execution.
3. Fetch or install declared dependencies.
4. Bind a local service to localhost.

Class B MUST NOT include arbitrary command execution or user-scope writes. A tool that can execute arbitrary commands is Class C.

### 2.3 Class C — Executable

Declares operations that:

1. Execute hooks, scripts or commands.
2. Run third-party code during install, apply, verify or launch.
3. Perform writes decided by executed code rather than by declared data.

Class C behavior MUST NOT be executed during preview. It requires explicit approval before it runs.

### 2.4 Class D — Privileged

Declares operations that:

1. Write to **user scope**.
2. Require administrative or elevated privileges.
3. Access, request or handle secrets or credentials.
4. Expose a local service beyond localhost, or otherwise grant broad system access.

---

## 3. Assignment rules

1. Every component and every permission MUST be assigned the **minimum class that covers all of its operations**. Under-classification is a conformance failure.
2. The **effective risk class** of a package is the maximum risk class over all its components and permissions. Risk classes are monotonic: composing classes yields the highest of them.
3. A risk class MUST NOT be lowered by documentation, by a claim of trustworthiness, or by the absence of an observed incident.
4. A class is a property of declared capability, not of provenance. A signed artifact with Class C behavior is still Class C.
5. When a component's operations are not fully known, it MUST be classified at the highest class it could plausibly reach; an unknown MUST NOT be treated as Passive.

---

## 4. Risk class and autonomy

1. The risk class sets the **minimum friction floor** for an operation, expressed as autonomy levels 0–3 (see [`glossary.md`](../glossary.md), "Autonomy level", and [`permissions.md`](permissions.md)).
2. Class A within project scope MAY proceed at a low autonomy level under a simple confirmation or policy.
3. Class B requires approval that reflects tooling access.
4. Class C and Class D require explicit confirmation. There MUST NOT be an absolute bypass, such as an unconditional `--yes`, capable of skipping them.
5. The operating system always remains the final authority for elevated operations.

---

## 5. Non-claims

1. A risk class MUST NOT be presented as a measure of trust, quality or safety.
2. A low risk class MUST NOT be used to imply that an artifact was reviewed, tested or verified. Those are separate signals (trust label and conformance).
3. A high risk class MUST NOT be used to imply that an artifact is malicious. It states capability, not intent.

---

## 6. Summary

- A Passive · B Tooling · C Executable · D Privileged.
- Each class is defined by the operations it declares; the floor is the highest operation present.
- Effective package risk is the maximum over parts, and is monotonic under composition.
- Risk class governs the minimum autonomy friction; there is no absolute bypass.
- Risk class is a capability statement, not a trust or safety claim.
