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
