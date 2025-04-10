# Setup Instructions

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Set up EA credentials ([see below](#set-up-ea-credentials))

## Set Up EA Credentials

It may look like a lot, but I just broke it down into many small, very easy steps.
(You will only need to do this every couple of months)

1. Log in to EA's webiste: [ea.com](https://www.ea.com/)
2. Open this URL in your browser: [accounts.ea.com](https://accounts.ea.com/connect/auth?client_id=ORIGIN_JS_SDK&response_type=token&redirect_uri=nucleus:rest&prompt=none&release_type=prod)
3. Open the Developer Console `Ctrl+Shift+I` (or `Right Click` then select `Inspect Element`)
4. Go to the `Application` tab
5. Select `cookies` on the left
6. Copy `remid` and `sid`
7. Put them both in the `.env` file (like `REMID="<remid>"`, separted by newlines)
8. (optional) You can put the `access_token` from the page in Step 2 in the `.env` as well

# System Installation (optional)

This allows you to use `ea_id` instead of `cargo run --`

```bash
cargo install --git https://github.com/MasterTemple/ea_id
```

# Usage

Note: Your `.env` file must be in the same directory that you run `ea_id` or `cargo run`

## Get User ID From Name

If installed,

Run `ea_id --id 1003036344058`

Otherwise,

Run `cargo run -- --id 1003036344058`

```md
┌───────────────────────┬───────────────┐
│ Field                 │ Value         │
├───────────────────────┼───────────────┤
│ user_id               │ 1003036344058 │
├───────────────────────┼───────────────┤
│ email                 │ None          │
├───────────────────────┼───────────────┤
│ persona_id            │ 1713374124    │
├───────────────────────┼───────────────┤
│ ea_id                 │ methodsorder  │
├───────────────────────┼───────────────┤
│ first_name            │ None          │
├───────────────────────┼───────────────┤
│ last_name             │ None          │
├───────────────────────┼───────────────┤
│ underage_user         │ false         │
├───────────────────────┼───────────────┤
│ is_discoverable_email │ false         │
└───────────────────────┴───────────────┘
```

## Get User Name From ID

If installed,

Run `ea_id --name methodsorder`

Otherwise,

Run `cargo run -- --name methodsorder`

```md
┌───────────────────────┬───────────────┐
│ Field                 │ Value         │
├───────────────────────┼───────────────┤
│ user_id               │ 1003036344058 │
├───────────────────────┼───────────────┤
│ email                 │ None          │
├───────────────────────┼───────────────┤
│ persona_id            │ 1713374124    │
├───────────────────────┼───────────────┤
│ ea_id                 │ methodsorder  │
├───────────────────────┼───────────────┤
│ first_name            │ None          │
├───────────────────────┼───────────────┤
│ last_name             │ None          │
├───────────────────────┼───────────────┤
│ underage_user         │ false         │
├───────────────────────┼───────────────┤
│ is_discoverable_email │ false         │
└───────────────────────┴───────────────┘
```
