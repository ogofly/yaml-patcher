[![MIT][s2]][l2] [![Latest Version][s1]][l1]

[s1]: https://img.shields.io/crates/v/yaml-patcher.svg
[l1]: https://crates.io/crates/yaml-patcher

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

A generic patcher for YAML files.

Given a base file and a patch file, yaml-patcher will output a modified file.

The patch file is a list of (`path`, `value`) where

- `path` is a space separated list of map keys or array indexes forming a path to the value
- `value` is any kind of YAML value, even structured, and will replace the old value

# Usage

```bash
yaml-patcher --base base.yml --patch patch.yml > patched.yml
```

# Example

This is a simplified example for a real use-case: modifying a Mkdocs file in a build chain.

## Base file

```yaml
root:
  first:
    - arr0: arr0-val
    - arr1: arr1-val
  second:
    - arr0: arr0-val
    - arr1: arr1-val
  third:
    a: A
    b: B
```

## Patch file

```yaml
# replace item
root first 1 arr1 : arr1-replace

# append array
root second + : 
  - arr00: arr00-append
  - arr11: arr00-append

# replace
root third b : B-replace
```

## Result

```bash
yaml-patcher -b ./example/base.yaml -p example/patch.yaml
```

```yaml
---
root:
  first:
    - arr0: arr0-val
    - arr1: arr1-replace
  second:
    - arr0: arr0-val
    - arr1: arr1-val
    - arr00: arr00-append
    - arr11: arr00-append
  third:
    a: A
    b: B-replace
```


# Features

- change existing values to new values
- append new value to array

I might add new features, like removing values, or adding them, or applying pattern based replacements.

If you need such feature please tell me.
