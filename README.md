# ![忍者](https://user-images.githubusercontent.com/90976669/236746701-9386295a-7116-4029-8e3d-7a7310029205.png) Net Ninjas 

> ⚠️ This is a work in progress.


A multiplayer bevy based game which can be played in the browser.


## How to build

### Native

```
$ cargo build --release
```

### Web

```
$ trunk build --no-default-features
```

> You can then serve the `index.html` file in the `dist` folder.

## Optional feature flags

- `editor`: Enables the editor UI with [bevy_editor_pls](https://crates.io/crates/bevy_editor_pls).
