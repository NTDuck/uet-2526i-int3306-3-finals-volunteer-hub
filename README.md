# uet-2526i-int3306-3-finals-volunteer-hub

## The project has 2 decoupled targets, SvelteKit (`./frontend/sveltekit-minimal`) and Vue/Express (`./frontend/vue-express`). The former is only for demonstration purposes and does not cover all Use Cases.

## Deps

- [Rust](https://rust-lang.org/tools/install/)<br> [Windows] use
  `stable-x86_64-pc-windows-msvc` toolchain
- [MSVC Build tools](https://visualstudio.microsoft.com/downloads/)<br>
  [Windows] use the following individual components: Windows Universal C
  Runtime, Windows 11 SDK (10.0.26100.7175), Windows Performance Toolkit, MSVC
  v143 - VS 2022 C++ ARM build tools (v14.44-17.14), MSVC v143 - VS 2022 C++
  x64/x86 build tools (v14.44-17.14)
- Clang<br> use LLVM
- [wasm-pack](https://drager.github.io/wasm-pack/)<br> run
  `cargo install wasm-pack`
- [Deno](https://docs.deno.com/runtime/getting_started/installation/)

## How to install

### For SvelteKit

```cmd
$ cd ./frontend/sveltekit-minimal && deno install
```

### For Vue

```cmd
$ cd ./frontend/vue-express && deno install
$ cd ./src/vue && npm install
```

## How to run

### Compile to WASM, on Windows

```cmd
$ deno task build-wasm
```

Manually copy `./package.json.example` as `package.json` to
`./backend/bindings/output/`.

### Compile to WASM, on Linux/MacOS

```cmd
$ deno task build-wasm && mkdir -p backend/bindings/output && echo '{"name": "volunteer-hub-wasm", "version": "1.0.0", "main": "volunteer-hub.js", "types": "volunteer-hub.d.ts", "files": ["volunteer-hub_bg.wasm", "volunteer-hub.js", "volunteer-hub.d.ts"]}' > backend/bindings/output/package.json
```

### Development (SvelteKit)

```cmd
$ deno task dev-sveltekit-minimal
```

### Production (SvelteKit)

```cmd
$ deno task build-sveltekit-minimal
$ deno task preview-sveltekit-minimal
```

### Development (Vue)

```
$ cd frontend/vue-express
$ deno task dev
$ deno task vue
```

## How to please Github

Assume we're merging PR branch `indev` into `master`. The following snippet
achieves the same thing as if we visit each conflict and select
`Keep Current Changes`.

```cmd
# https://trunk.io/blog/git-commit-messages-are-useless
$ git config --global alias.nccommit "commit -a --allow-empty-message -m ''"

$ git pull origin master
$ git checkout indev
$ git merge master
$ git checkout --ours .
$ git add .
$ git nccommit
$ git push -u origin indev
```

## Use cases

> https://itest.com.vn/lects/webappdev/mockproj/VolunteerHub.htm

## TODO

- Increase parallelism via e.g. `buffer_unordered(...)` for streams, `rayon`
- Replace `.filter_map(|transposable| async move { transposable.transpose() })`
  with `.transpose()` via an `axiom` trait
- Improve performance by collecting into `::smallvec::SmallVec` instead of
  `::std::vec::Vec` in hot loops
- Improve performance by querying specific column(s)
