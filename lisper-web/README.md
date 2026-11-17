# Lisper web app

The Next.js frontend for [Lisper](../README.md). It evaluates Lisp expressions in the browser through the Rust interpreter compiled to WebAssembly.

## Requirements

- Node.js 26 (see `.nvmrc`)
- Rust with the `wasm32-unknown-unknown` target
- `wasm-pack` 0.15.1

## Local development

From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cd wasm
wasm-pack build --out-dir ../lisper-web/pkg --target web
cd ..
mkdir -p lisper-web/public/wasm
cp lisper-web/pkg/lisper_wasm.js lisper-web/pkg/lisper_wasm_bg.wasm lisper-web/public/wasm/
cd lisper-web
npm ci
npm run dev
```

Open [http://localhost:3000](http://localhost:3000).

The browser loads `/wasm/lisper_wasm.js` as a native ES module, which initializes `/wasm/lisper_wasm_bg.wasm`. The generated files must be copied into `public/wasm/` after rebuilding the Rust WASM wrapper. The generated `pkg/` directory is also used as the local `lisper-wasm` npm package.

## Validation

From the repository root:

```sh
cargo test --workspace --locked
cd e2e
npm ci
npx playwright install chromium
npm test
```

The Playwright configuration starts the Next.js development server automatically. See [browser test instructions](../e2e/README.md).

## Deployment

GitHub Actions builds the WASM assets and deploys the Next.js application to Vercel. See `.github/workflows/` for the current workflow configuration.
