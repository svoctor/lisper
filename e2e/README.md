# Browser end-to-end tests

These Playwright tests exercise the real Next.js frontend and Rust WebAssembly interpreter in Chromium.

## Local setup

From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
cd wasm && wasm-pack build --out-dir ../lisper-web/pkg --target bundler && cd ..
cd lisper-web && npm install && cd ..
cd e2e && npm install && npx playwright install chromium && npm test
```

For interactive debugging use `npm run test:ui` from `e2e/`. Playwright starts the web development server automatically. To test against a manually running development server, leave it on port 3000; the local configuration reuses it.

CI uses the same tests and uploads traces/screenshots on failures. The separate `e2e/package.json` keeps Playwright isolated from the legacy Next.js 12 dependency tree until the frontend migration. Regenerate and commit npm lockfiles when dependencies are modernized.
