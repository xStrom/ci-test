## Fairly reproducible runs

If a CI run succeeds, then reruns should in most cases also succeed.
The idea being that the CI should be testing changes in a specific repository, not for changes in its own tools.
When a PR arrives, the CI should react to only those changes and not fail by default because it itself has become stale.

This means that completely unversioned tool usage that is so common in CI scripts is not acceptable.
We can't just use `ubuntu-latest` and hope that the next OS version is backwards compatible with everything we depend on.
Surfing on the stable Rust toolchain wave just guarantees scheduled CI failure for innocent PRs.
Similar story applies to even smaller dependencies.

That said, we are still interested in critical bug fixes.
Those might be of the security variety, or they might be more mundane.

With OS versions we don't have much of a choice on this matter when using GitHub provided runners.
At most we can specify e.g. `ubuntu-22.04` but that image will still be [updated weekly][gh-weekly-images].

With smaller dependencies we aim for automatic [SemVer] compatible updates.
This allows for bug fixes to propagate even without updating the CI scripts.
However, in cases where even SemVer compatible updates can cause CI failures, the version requirements are pinned.

With the Rust toolchain we specify `MAJOR.MINOR` and allow for `PATCH` releases to propagate automatically.
This is because we treat many warnings as errors.
So even though a new `MINOR` release will compile old Rust code, it usually generates new warnings which would result in CI failure.

## Cross compiling doc tests

In order to cross compile doc tests, i.e. code blocks inside documentation, we need to use the [`-Zdoctest-xcompile`] feature.
This feature is [not yet stabilized][doctest-xcompile-issue] and is officially only available via the nightly toolchain.

We don't really want to use the latest nightly to compile our tests.
It can contain behavior differences compared to what our code expects.
The daily nature of its release cycle can also cause intermittent compilation issues in the CI.
What we would like is a pinned nightly version that basically matches our pinned stable version as much as possible.

The problem we run into is that there isn't any nightly version that matches the behavior of the stable toolchain.
The stable version is branched from `master` 7 weeks before release.
That means the stable version has 7 weeks worth of final fixes which the nightly version from 7 weeks prior doesn't have.
The nightly version from the day the stable version was published might contain those fixes, but it also contains a bunch of behavior from future releases.

Luckily there is a workaround.
We can set the `RUSTC_BOOTSTRAP` environment variable to `1`.
This is officially not meant for anything other than bootstrapping the compiler.
However in practice it is just what we need, as it allows using unstable features with the stable toolchain.

## Matching Docs.rs when building documentation

We need to use the latest nightly toolchain because that is what [Docs.rs does][docsrs-build].
We also need to make sure that we test all features and pass the `docsrs` cfg flag.
Docs.rs cross compiles all targets except `x86_64-unknown-linux-gnu` so we do the same.

## Passing arbitrary inputs to `bash`

We can't directly use e.g. `${{ inputs.desc }}` in a `bash` script because we can't escape special characters.
Instead we must pass it via an environment variable.

```sh
# inputs:
#   desc:
#     default: It's "cool"
# env:
#   ARG_DESC: ${{ inputs.desc }}
echo ${{ inputs.desc }}   # -> echo It's "cool"   .. which is invalid syntax
echo '${{ inputs.desc }}' # -> echo 'It's "cool"' .. which is invalid syntax
echo "${{ inputs.desc }}" # -> echo "It's "cool"" .. valid but prints the wrong value: It's cool
echo "$ARG_DESC"          # -> echo "$ARG_DESC"   .. valid and prints the right value: It's "cool"
```

If we expect a known format from an input (e.g. package name, target triple) then it's fine to use it directly.
Malformed input is not a security concern, but a functionality concern.
This of course assumes that we don't source inputs from outside the repository, which would not have passed review.

[`-Zdoctest-xcompile`]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#doctest-xcompile
[doctest-xcompile-issue]: https://github.com/rust-lang/rust/issues/64245
[docsrs-build]: https://docs.rs/about/builds
[SemVer]: https://semver.org/
[gh-weekly-images]: https://github.com/actions/runner-images#ga
