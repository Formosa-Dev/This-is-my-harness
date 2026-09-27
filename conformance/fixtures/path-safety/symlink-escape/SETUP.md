# symlink-escape fixture setup

The manifest declares a component at `escape-link`, a lexically safe path. The
escape is only observable once `escape-link` resolves, through a symlink, to a
target **outside** the package root.

The committed tree intentionally contains no symlink: Git does not store them
reliably on every platform (on Windows `core.symlinks` is often `false`), so a
committed link would materialize as a plain file or directory and the fixture
would stop proving anything.

The conformance runner (`packages/validator/tests/conformance.rs`) therefore
materializes the case at run time:

1. It tries to create a real directory symlink for `escape-link` pointing to a
   temporary directory outside the package root. When the host allows it, the
   fixture is evaluated through the real filesystem layer.
2. When the host does not allow symlink creation (for example Windows without
   Developer Mode), the runner exercises the same containment logic through the
   filesystem layer's public injected-resolver seam (`pathsafe::check_with_resolver`)
   with a synthetic link, and records that it did so.

Both paths assert the same diagnostic: `path.symlink_escape` at
`/spec/components/0/path`. The fixture is expected to be rejected either way.
