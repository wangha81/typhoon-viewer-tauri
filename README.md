# Typhoon Viewer 

A typhoon tracer with techs below

## Tech stack
- [Tauri](https://tauri.app/) ( Rust )
- Vue 3
- TypeScript
- Cesium.js

## Prerequisite
- Rust
- Node

## How to start
- follow the [prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- Install [Node](https://nodejs.org/en/) with [nvm](https://github.com/nvm-sh/nvm) (recommend)
    - `yarn` is also recommended for node
- `yarn tauri dev` for dev
- `yarn tauri build` for build with your own platform
- `yarn tauri` for tauri info
- `yarn dev` for frontend only
- more info in `package.json`

## Data reference
[RSMC Tokyo - Typhoon Center](https://www.jma.go.jp/jma/jma-eng/jma-center/rsmc-hp-pub-eg/besttrack.html)

## Demo
![image](./demo.gif)


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
