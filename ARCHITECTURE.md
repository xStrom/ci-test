## Version numbers

When new versions of the CI scripts are released, the following holds:

* `MAJOR` - Anything goes, carefully reading the changelog is required and changes to the calling CI script are likely.
* `MINOR` - Guaranteed backwards compatible API which means no calling CI script changes are required.
            CI failure conditions have likely changed, so changes to the calling repository might be required.
* `PATCH` - Guaranteed stable API and unchanged CI failure conditions. Trivial upgrade likely.

For the releases where `MAJOR` is still zero we shift the importance of `MINOR` and `PATCH` up by one and have no functional `PATCH` number.
Which is to say that instead of `MAJOR.MINOR.PATCH` we have effectively `0.MAJOR.MINOR`.

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

## Testing individual features despite Cargo's feature unification

We don't use `--all-targets` because then even `--lib` and `--bins` are compiled with dev dependencies enabled, which does not match how they would be compiled by users.
A dev dependency might transitively enable a feature that we need for a regular dependency, and checking with `--all-targets` would not find our feature requirements lacking.
This problem still applies to Cargo resolver version 3.
Thus we split all the targets into two steps, one with `--lib --bins` and another with `--tests --benches --examples`.
Also, we can't actually give `--lib --bins` explicitly because then Cargo will error on binary-only packages.
Luckily the default behavior of Cargo with no explicit targets is the same as with `--lib --bins` but without the error.

We use [`cargo-hack`] for a similar reason.
Cargo's `--workspace` will do feature unification across the whole workspace.
So a dependency might have a feature enabled by just one package, while being used by multiple packages.
This is solved by `cargo-hack` by dealing with each package separately.

Using `cargo-hack` also allows us to more easily test the feature matrix of our packages.
We use `--optional-deps --each-feature` which will run a separate check for every feature.

## Ensuring MSRV is correctly defined

Linebender projects always define their MSRV in the root `Cargo.toml` via thea `rust-version` property.
The MSRV jobs run only `cargo check` because different Clippy versions can disagree on goals and running tests introduces dev dependencies which may require a higher MSRV than the bare package.
Checking is limited to packages that are intended for publishing (`publish = true`) to keep MSRV as low as possible.

## Conditional compilation

Every CI workflow that performs compilation supports the `target` input variable.
This way every potential target can be verified, including any target specific code.

Additionally, if the workspace uses [`debug_assertions`] then we verify all code twice, with it set to `true` or `false`.
We always keep it `true` for external dependencies so that we can reuse the cache for faster builds.
This does mean that doing a manual `--release` build before publishing is worth it and may reveal rare issues.

## Running tests

We use [`cargo-nextest`], which has a faster concurrency model for running tests.
However [`cargo-nextest` does not support running doc tests][nextest-no-doc-tests], so we also have a `cargo test --doc` step.

## Cross compiling doc tests

In order to cross compile doc tests, i.e. code blocks inside documentation, we need to use the [`-Zdoctest-xcompile`] feature.
This feature is [not yet stabilized][doctest-xcompile-issue] and is officially only available via the nightly toolchain.

We don't really want to use the latest nightly toolchain to compile our tests.
It can contain behavior differences compared to what our code expects.
The daily nature of its release cycle can also cause intermittent compilation issues in the CI.
What we would like is a pinned nightly version that basically matches our pinned stable version as much as possible.

The problem we run into is that there isn't any nightly version that matches the behavior of the stable toolchain.
The stable version is branched from `master` 7 weeks before release.
That means the stable version has 7 weeks worth of final fixes which the nightly version from 7 weeks prior doesn't have.
The nightly version from the day the stable version was published might contain those fixes, but it also contains a bunch of behavior from future releases.

Luckily there is a workaround.
We can set the [`RUSTC_BOOTSTRAP`] environment variable to `1`.
This is officially not meant for anything other than bootstrapping the compiler.
However in practice it is just what we need, as it allows using unstable features with the stable toolchain.

## Matching Docs.rs when building documentation

We need to use the latest nightly toolchain because that is what [Docs.rs does][docsrs-build].
We also need to make sure that we test all features and pass the `docsrs` cfg flag.
Docs.rs cross compiles all targets except `x86_64-unknown-linux-gnu` so we do the same.

## Caching issues

We don't save caches when the CI was triggered by the merge queue, because those caches will never be re-used.
That is, apart from the rare cases where there are multiple PRs in the merge queue.
This is because [GitHub doesn't share caches between merge queues and the main branch][queue-cache-issue].
To still be able to prime the cache, we trigger another CI run on `push` to `main` with the same commit that just had a CI run in `merge_group`.

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
Malicious actors could just as well add a new custom bash script to the CI in their PR.

[`-Zdoctest-xcompile`]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#doctest-xcompile
[doctest-xcompile-issue]: https://github.com/rust-lang/rust/issues/64245
[docsrs-build]: https://docs.rs/about/builds
[SemVer]: https://semver.org/
[gh-weekly-images]: https://github.com/actions/runner-images#ga
[`cargo-hack`]: https://github.com/taiki-e/cargo-hack
[`debug_assertions`]: https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions
[queue-cache-issue]: https://github.com/orgs/community/discussions/66430
[`cargo-nextest`]: https://nexte.st/
[nextest-no-doc-tests]: https://github.com/nextest-rs/nextest/issues/16
[`RUSTC_BOOTSTRAP`]: https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html#complications-of-bootstrapping
