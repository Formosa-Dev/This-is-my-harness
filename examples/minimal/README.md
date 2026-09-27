# example-minimal

The smallest valid harness package. It contains exactly one canonical manifest
and an empty `spec`, so it is passive by construction: it declares no
executable, tooling or privileged behavior and therefore needs no permissions.

The `instructions/` and `skills/` directories are optional content; the package
stays valid without them. They are present to show the conventional layout.
