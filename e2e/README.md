# Browser end-to-end tests

These Playwright tests exercise the real Next.js frontend and Rust WebAssembly interpreter in Chromium, including responsive viewport checks.

## Local setup

Use Node.js 26, Rust with the `wasm32-unknown-unknown` target, and `wasm-pack` 0.15.1.

From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cd wasm
wasm-pack build --out-dir ../lisper-web/pkg --target web
cd ..
mkdir -p lisper-web/public/wasm
cp lisper-web/pkg/lisper_wasm.js lisper-web/pkg/lisper_wasm_bg.wasm lisper-web/public/wasm/
cd lisper-web && npm ci && cd ..
cd e2e
npm ci
npx playwright install chromium
npm test
```

For interactive debugging, run `npm run test:ui` from `e2e/`. Playwright starts the web development server automatically. To test against a manually running development server, leave it on port 3000; the local configuration reuses it.

CI runs the Rust checks and browser tests and uploads traces/screenshots when browser tests fail. Playwright uses a separate `e2e/package.json` and lockfile so browser-test dependencies are managed independently of the frontend.
