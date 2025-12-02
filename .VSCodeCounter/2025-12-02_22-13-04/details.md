# Details

Date : 2025-12-02 22:13:04

Directory d:\\root\\dev\\js\\volunteer-hub

Total : 110 files,  9820 codes, 228 comments, 1647 blanks, all 11695 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [.github/workflows/cargo-build.yml](/.github/workflows/cargo-build.yml) | YAML | 30 | 0 | 9 | 39 |
| [.github/workflows/cargo-ci.yml](/.github/workflows/cargo-ci.yml) | YAML | 31 | 0 | 8 | 39 |
| [.github/workflows/cargo-dependencies-check.yml](/.github/workflows/cargo-dependencies-check.yml) | YAML | 52 | 3 | 15 | 70 |
| [.github/workflows/cargo-format-and-lint.yml](/.github/workflows/cargo-format-and-lint.yml) | YAML | 47 | 0 | 14 | 61 |
| [.github/workflows/cargo-test.yml](/.github/workflows/cargo-test.yml) | YAML | 33 | 0 | 10 | 43 |
| [.github/workflows/sveltekit-minimal-ci.yml](/.github/workflows/sveltekit-minimal-ci.yml) | YAML | 11 | 4 | 5 | 20 |
| [.github/workflows/sveltekit-minimal-format-and-lint.yml](/.github/workflows/sveltekit-minimal-format-and-lint.yml) | YAML | 38 | 0 | 12 | 50 |
| [README.md](/README.md) | Markdown | 208 | 0 | 27 | 235 |
| [backend/.cargo/config.toml](/backend/.cargo/config.toml) | TOML | 2 | 0 | 1 | 3 |
| [backend/.clippy.toml](/backend/.clippy.toml) | TOML | 9 | 71 | 2 | 82 |
| [backend/.rustfmt.toml](/backend/.rustfmt.toml) | TOML | 31 | 47 | 2 | 80 |
| [backend/Cargo.lock](/backend/Cargo.lock) | TOML | 1,521 | 2 | 176 | 1,699 |
| [backend/Cargo.toml](/backend/Cargo.toml) | TOML | 40 | 7 | 8 | 55 |
| [backend/aliases/Cargo.toml](/backend/aliases/Cargo.toml) | TOML | 14 | 0 | 3 | 17 |
| [backend/aliases/src/lib.rs](/backend/aliases/src/lib.rs) | Rust | 26 | 1 | 6 | 33 |
| [backend/axiom-derive/Cargo.toml](/backend/axiom-derive/Cargo.toml) | TOML | 18 | 0 | 5 | 23 |
| [backend/axiom-derive/src/erratum.rs](/backend/axiom-derive/src/erratum.rs) | Rust | 271 | 1 | 48 | 320 |
| [backend/axiom-derive/src/lib.rs](/backend/axiom-derive/src/lib.rs) | Rust | 10 | 27 | 4 | 41 |
| [backend/axiom-derive/src/verifiable.rs](/backend/axiom-derive/src/verifiable.rs) | Rust | 111 | 0 | 27 | 138 |
| [backend/axiom/Cargo.toml](/backend/axiom/Cargo.toml) | TOML | 17 | 0 | 4 | 21 |
| [backend/axiom/src/lib.rs](/backend/axiom/src/lib.rs) | Rust | 209 | 5 | 36 | 250 |
| [backend/bindings/wasm-bindings/Cargo.toml](/backend/bindings/wasm-bindings/Cargo.toml) | TOML | 35 | 1 | 8 | 44 |
| [backend/bindings/wasm-bindings/src/build.rs](/backend/bindings/wasm-bindings/src/build.rs) | Rust | 23 | 2 | 5 | 30 |
| [backend/bindings/wasm-bindings/src/lib.rs](/backend/bindings/wasm-bindings/src/lib.rs) | Rust | 139 | 5 | 23 | 167 |
| [backend/core/domain/Cargo.toml](/backend/core/domain/Cargo.toml) | TOML | 16 | 0 | 4 | 20 |
| [backend/core/domain/src/lib.rs](/backend/core/domain/src/lib.rs) | Rust | 205 | 10 | 35 | 250 |
| [backend/core/use-cases/Cargo.toml](/backend/core/use-cases/Cargo.toml) | TOML | 26 | 0 | 6 | 32 |
| [backend/core/use-cases/src/boundaries/create\_comment.rs](/backend/core/use-cases/src/boundaries/create_comment.rs) | Rust | 77 | 0 | 17 | 94 |
| [backend/core/use-cases/src/boundaries/create\_event.rs](/backend/core/use-cases/src/boundaries/create_event.rs) | Rust | 88 | 0 | 18 | 106 |
| [backend/core/use-cases/src/boundaries/create\_post.rs](/backend/core/use-cases/src/boundaries/create_post.rs) | Rust | 82 | 0 | 18 | 100 |
| [backend/core/use-cases/src/boundaries/create\_reaction.rs](/backend/core/use-cases/src/boundaries/create_reaction.rs) | Rust | 74 | 0 | 16 | 90 |
| [backend/core/use-cases/src/boundaries/export\_events.rs](/backend/core/use-cases/src/boundaries/export_events.rs) | Rust | 106 | 1 | 16 | 123 |
| [backend/core/use-cases/src/boundaries/export\_volunteers.rs](/backend/core/use-cases/src/boundaries/export_volunteers.rs) | Rust | 106 | 1 | 16 | 123 |
| [backend/core/use-cases/src/boundaries/mod.rs](/backend/core/use-cases/src/boundaries/mod.rs) | Rust | 79 | 0 | 3 | 82 |
| [backend/core/use-cases/src/boundaries/moderate\_event.rs](/backend/core/use-cases/src/boundaries/moderate_event.rs) | Rust | 106 | 0 | 18 | 124 |
| [backend/core/use-cases/src/boundaries/moderate\_event\_registration.rs](/backend/core/use-cases/src/boundaries/moderate_event_registration.rs) | Rust | 111 | 0 | 19 | 130 |
| [backend/core/use-cases/src/boundaries/moderate\_user.rs](/backend/core/use-cases/src/boundaries/moderate_user.rs) | Rust | 107 | 0 | 18 | 125 |
| [backend/core/use-cases/src/boundaries/remove\_comment.rs](/backend/core/use-cases/src/boundaries/remove_comment.rs) | Rust | 74 | 0 | 16 | 90 |
| [backend/core/use-cases/src/boundaries/remove\_event.rs](/backend/core/use-cases/src/boundaries/remove_event.rs) | Rust | 95 | 0 | 18 | 113 |
| [backend/core/use-cases/src/boundaries/remove\_post.rs](/backend/core/use-cases/src/boundaries/remove_post.rs) | Rust | 74 | 0 | 16 | 90 |
| [backend/core/use-cases/src/boundaries/remove\_reaction.rs](/backend/core/use-cases/src/boundaries/remove_reaction.rs) | Rust | 76 | 0 | 17 | 93 |
| [backend/core/use-cases/src/boundaries/sign\_in.rs](/backend/core/use-cases/src/boundaries/sign_in.rs) | Rust | 81 | 0 | 13 | 94 |
| [backend/core/use-cases/src/boundaries/sign\_up.rs](/backend/core/use-cases/src/boundaries/sign_up.rs) | Rust | 79 | 0 | 16 | 95 |
| [backend/core/use-cases/src/boundaries/subscribe\_to\_event.rs](/backend/core/use-cases/src/boundaries/subscribe_to_event.rs) | Rust | 100 | 0 | 18 | 118 |
| [backend/core/use-cases/src/boundaries/unsubscribe\_from\_event.rs](/backend/core/use-cases/src/boundaries/unsubscribe_from_event.rs) | Rust | 100 | 0 | 18 | 118 |
| [backend/core/use-cases/src/boundaries/update\_comment.rs](/backend/core/use-cases/src/boundaries/update_comment.rs) | Rust | 79 | 0 | 17 | 96 |
| [backend/core/use-cases/src/boundaries/update\_event.rs](/backend/core/use-cases/src/boundaries/update_event.rs) | Rust | 117 | 0 | 23 | 140 |
| [backend/core/use-cases/src/boundaries/update\_post.rs](/backend/core/use-cases/src/boundaries/update_post.rs) | Rust | 84 | 0 | 19 | 103 |
| [backend/core/use-cases/src/boundaries/view\_event\_channel.rs](/backend/core/use-cases/src/boundaries/view_event_channel.rs) | Rust | 142 | 0 | 24 | 166 |
| [backend/core/use-cases/src/boundaries/view\_event\_history.rs](/backend/core/use-cases/src/boundaries/view_event_history.rs) | Rust | 148 | 0 | 20 | 168 |
| [backend/core/use-cases/src/boundaries/view\_event\_recommendation.rs](/backend/core/use-cases/src/boundaries/view_event_recommendation.rs) | Rust | 101 | 0 | 16 | 117 |
| [backend/core/use-cases/src/boundaries/view\_event\_volunteers.rs](/backend/core/use-cases/src/boundaries/view_event_volunteers.rs) | Rust | 148 | 0 | 21 | 169 |
| [backend/core/use-cases/src/boundaries/view\_events.rs](/backend/core/use-cases/src/boundaries/view_events.rs) | Rust | 177 | 0 | 24 | 201 |
| [backend/core/use-cases/src/boundaries/view\_post.rs](/backend/core/use-cases/src/boundaries/view_post.rs) | Rust | 161 | 0 | 27 | 188 |
| [backend/core/use-cases/src/boundaries/view\_published\_events.rs](/backend/core/use-cases/src/boundaries/view_published_events.rs) | Rust | 156 | 0 | 21 | 177 |
| [backend/core/use-cases/src/boundaries/view\_users.rs](/backend/core/use-cases/src/boundaries/view_users.rs) | Rust | 178 | 0 | 25 | 203 |
| [backend/core/use-cases/src/gateways.rs](/backend/core/use-cases/src/gateways.rs) | Rust | 353 | 0 | 80 | 433 |
| [backend/core/use-cases/src/interactors/create\_comment.rs](/backend/core/use-cases/src/interactors/create_comment.rs) | Rust | 67 | 0 | 18 | 85 |
| [backend/core/use-cases/src/interactors/create\_event.rs](/backend/core/use-cases/src/interactors/create_event.rs) | Rust | 73 | 0 | 19 | 92 |
| [backend/core/use-cases/src/interactors/create\_post.rs](/backend/core/use-cases/src/interactors/create_post.rs) | Rust | 80 | 0 | 19 | 99 |
| [backend/core/use-cases/src/interactors/create\_reaction.rs](/backend/core/use-cases/src/interactors/create_reaction.rs) | Rust | 59 | 0 | 14 | 73 |
| [backend/core/use-cases/src/interactors/export\_events.rs](/backend/core/use-cases/src/interactors/export_events.rs) | Rust | 37 | 0 | 8 | 45 |
| [backend/core/use-cases/src/interactors/export\_volunteers.rs](/backend/core/use-cases/src/interactors/export_volunteers.rs) | Rust | 37 | 0 | 8 | 45 |
| [backend/core/use-cases/src/interactors/mod.rs](/backend/core/use-cases/src/interactors/mod.rs) | Rust | 56 | 0 | 2 | 58 |
| [backend/core/use-cases/src/interactors/moderate\_event.rs](/backend/core/use-cases/src/interactors/moderate_event.rs) | Rust | 71 | 0 | 16 | 87 |
| [backend/core/use-cases/src/interactors/moderate\_event\_registration.rs](/backend/core/use-cases/src/interactors/moderate_event_registration.rs) | Rust | 85 | 0 | 18 | 103 |
| [backend/core/use-cases/src/interactors/moderate\_user.rs](/backend/core/use-cases/src/interactors/moderate_user.rs) | Rust | 71 | 0 | 17 | 88 |
| [backend/core/use-cases/src/interactors/remove\_comment.rs](/backend/core/use-cases/src/interactors/remove_comment.rs) | Rust | 51 | 0 | 11 | 62 |
| [backend/core/use-cases/src/interactors/remove\_event.rs](/backend/core/use-cases/src/interactors/remove_event.rs) | Rust | 59 | 0 | 16 | 75 |
| [backend/core/use-cases/src/interactors/remove\_post.rs](/backend/core/use-cases/src/interactors/remove_post.rs) | Rust | 51 | 0 | 11 | 62 |
| [backend/core/use-cases/src/interactors/remove\_reaction.rs](/backend/core/use-cases/src/interactors/remove_reaction.rs) | Rust | 51 | 0 | 12 | 63 |
| [backend/core/use-cases/src/interactors/sign\_in.rs](/backend/core/use-cases/src/interactors/sign_in.rs) | Rust | 66 | 0 | 13 | 79 |
| [backend/core/use-cases/src/interactors/sign\_up.rs](/backend/core/use-cases/src/interactors/sign_up.rs) | Rust | 63 | 0 | 19 | 82 |
| [backend/core/use-cases/src/interactors/subscribe\_to\_event.rs](/backend/core/use-cases/src/interactors/subscribe_to_event.rs) | Rust | 71 | 0 | 18 | 89 |
| [backend/core/use-cases/src/interactors/unsubscribe\_from\_event.rs](/backend/core/use-cases/src/interactors/unsubscribe_from_event.rs) | Rust | 53 | 0 | 14 | 67 |
| [backend/core/use-cases/src/interactors/update\_comment.rs](/backend/core/use-cases/src/interactors/update_comment.rs) | Rust | 65 | 0 | 18 | 83 |
| [backend/core/use-cases/src/interactors/update\_event.rs](/backend/core/use-cases/src/interactors/update_event.rs) | Rust | 89 | 0 | 23 | 112 |
| [backend/core/use-cases/src/interactors/update\_post.rs](/backend/core/use-cases/src/interactors/update_post.rs) | Rust | 71 | 0 | 19 | 90 |
| [backend/core/use-cases/src/interactors/view\_event\_channel.rs](/backend/core/use-cases/src/interactors/view_event_channel.rs) | Rust | 86 | 0 | 14 | 100 |
| [backend/core/use-cases/src/interactors/view\_event\_history.rs](/backend/core/use-cases/src/interactors/view_event_history.rs) | Rust | 58 | 0 | 12 | 70 |
| [backend/core/use-cases/src/interactors/view\_event\_recommendation.rs](/backend/core/use-cases/src/interactors/view_event_recommendation.rs) | Rust | 61 | 0 | 11 | 72 |
| [backend/core/use-cases/src/interactors/view\_event\_volunteers.rs](/backend/core/use-cases/src/interactors/view_event_volunteers.rs) | Rust | 58 | 0 | 12 | 70 |
| [backend/core/use-cases/src/interactors/view\_events.rs](/backend/core/use-cases/src/interactors/view_events.rs) | Rust | 56 | 0 | 10 | 66 |
| [backend/core/use-cases/src/interactors/view\_post.rs](/backend/core/use-cases/src/interactors/view_post.rs) | Rust | 82 | 0 | 11 | 93 |
| [backend/core/use-cases/src/interactors/view\_published\_events.rs](/backend/core/use-cases/src/interactors/view_published_events.rs) | Rust | 56 | 0 | 10 | 66 |
| [backend/core/use-cases/src/interactors/view\_users.rs](/backend/core/use-cases/src/interactors/view_users.rs) | Rust | 51 | 0 | 10 | 61 |
| [backend/core/use-cases/src/lib.rs](/backend/core/use-cases/src/lib.rs) | Rust | 3 | 0 | 1 | 4 |
| [backend/infrastructures/Cargo.toml](/backend/infrastructures/Cargo.toml) | TOML | 26 | 5 | 6 | 37 |
| [backend/infrastructures/src/lib.rs](/backend/infrastructures/src/lib.rs) | Rust | 185 | 3 | 51 | 239 |
| [deno.json](/deno.json) | JSON | 22 | 0 | 2 | 24 |
| [deno.lock](/deno.lock) | JSON | 17 | 0 | 1 | 18 |
| [frontend/sveltekit-minimal/README.md](/frontend/sveltekit-minimal/README.md) | Markdown | 27 | 0 | 15 | 42 |
| [frontend/sveltekit-minimal/deno.lock](/frontend/sveltekit-minimal/deno.lock) | JSON | 826 | 0 | 1 | 827 |
| [frontend/sveltekit-minimal/package.json](/frontend/sveltekit-minimal/package.json) | JSON | 28 | 0 | 1 | 29 |
| [frontend/sveltekit-minimal/src/app.d.ts](/frontend/sveltekit-minimal/src/app.d.ts) | TypeScript | 5 | 7 | 2 | 14 |
| [frontend/sveltekit-minimal/src/app.html](/frontend/sveltekit-minimal/src/app.html) | HTML | 11 | 0 | 1 | 12 |
| [frontend/sveltekit-minimal/src/lib/server/index.ts](/frontend/sveltekit-minimal/src/lib/server/index.ts) | TypeScript | 9 | 1 | 3 | 13 |
| [frontend/sveltekit-minimal/src/routes/(authed)/+layout.server.ts](/frontend/sveltekit-minimal/src/routes/(authed)/+layout.server.ts) | TypeScript | 7 | 0 | 2 | 9 |
| [frontend/sveltekit-minimal/src/routes/(authed)/+layout.svelte](/frontend/sveltekit-minimal/src/routes/(authed)/+layout.svelte) | Svelte | 7 | 0 | 3 | 10 |
| [frontend/sveltekit-minimal/src/routes/(authed)/dashboard/+page.svelte](/frontend/sveltekit-minimal/src/routes/(authed)/dashboard/+page.svelte) | Svelte | 1 | 0 | 1 | 2 |
| [frontend/sveltekit-minimal/src/routes/+layout.svelte](/frontend/sveltekit-minimal/src/routes/+layout.svelte) | Svelte | 9 | 0 | 3 | 12 |
| [frontend/sveltekit-minimal/src/routes/+page.svelte](/frontend/sveltekit-minimal/src/routes/+page.svelte) | Svelte | 1 | 0 | 1 | 2 |
| [frontend/sveltekit-minimal/src/routes/login/+page.server.ts](/frontend/sveltekit-minimal/src/routes/login/+page.server.ts) | TypeScript | 20 | 3 | 7 | 30 |
| [frontend/sveltekit-minimal/src/routes/login/+page.svelte](/frontend/sveltekit-minimal/src/routes/login/+page.svelte) | Svelte | 24 | 0 | 7 | 31 |
| [frontend/sveltekit-minimal/src/routes/logout/+page.server.ts](/frontend/sveltekit-minimal/src/routes/logout/+page.server.ts) | TypeScript | 9 | 0 | 2 | 11 |
| [frontend/sveltekit-minimal/src/routes/register/+page.server.ts](/frontend/sveltekit-minimal/src/routes/register/+page.server.ts) | TypeScript | 22 | 0 | 6 | 28 |
| [frontend/sveltekit-minimal/src/routes/register/+page.svelte](/frontend/sveltekit-minimal/src/routes/register/+page.svelte) | Svelte | 37 | 0 | 10 | 47 |
| [frontend/sveltekit-minimal/svelte.config.js](/frontend/sveltekit-minimal/svelte.config.js) | JavaScript JSX | 9 | 9 | 4 | 22 |
| [frontend/sveltekit-minimal/tsconfig.json](/frontend/sveltekit-minimal/tsconfig.json) | JSON with Comments | 14 | 11 | 1 | 26 |
| [frontend/sveltekit-minimal/vite.config.ts](/frontend/sveltekit-minimal/vite.config.ts) | TypeScript | 35 | 1 | 2 | 38 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)