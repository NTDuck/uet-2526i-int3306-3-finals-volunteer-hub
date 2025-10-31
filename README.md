# uet-2526i-int3306-3-finals-volunteer-hub

## Deps

- [Rust](https://rust-lang.org/tools/install/)
- [wasm-pack](https://drager.github.io/wasm-pack/) basically
  `cargo install wasm-pack`
- [Deno](https://docs.deno.com/runtime/getting_started/installation/)

## How to install

```cmd
cd ./frontend/sveltekit-minimal && deno install
```

## How to run

```cmd
deno task build-wasm
```

Development

```cmd
deno task dev-sveltekit-minimal
```

Production

```cmd
deno task build-sveltekit-minimal
deno task preview-sveltekit-minimal
```

# Notes

- Domain
  - Volunteer
  - Manager
  - Administrator

  - Event: name, date(time?), location, description, category, channel
  - EventChannel (like Facebook wall)
- Use cases
  - Common
    - Create account
    - Sign in (email, pw)
    - Dashboard
      - View events: newly announced/created?, having new posts, popular events
        (increasing fast: members, likes)
  - Volunteer
    - View events + by chrono/category
    - Subcribe to event (recv confirmation noti) // simple return, noti doesnt
      persist
    - Unsubscribe to event (before it happens)
    - View participated events + completion status
    - || Web push API for certain momentos (subscribed approved, completion
      status)
    - Post on EventChannel, Comment on Post, Like on Post (all of those after
      event is approved)

  - Event Manager
    - Create, Update, Delete Event + Input Validation stuff
    - Approve / Disapprove Volunteer's subscription
    - Update Volunteer's completion status
    - View voluteers
    - Post on EventChannel, Comment on Post, Like on Post (all of those after
      event is approved)

  - Administrator
    - Approve, Delete Event (that are created by Manager)
    - View, lock/unlocked V/EM accs
    - Export Events/Vs (CSV/JSON)
    -
