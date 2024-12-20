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
