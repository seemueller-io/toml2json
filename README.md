# toml2json

This project provides a web-based TOML to JSON converter. Conversion is handled client-side.

## Prerequisites
- [node/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [rust/cargo](https://www.rust-lang.org/tools/install)

## Quickstart
```bash
git clone  https://github.com/seemueller-io/toml2json.git
cd toml2json
# Install dependencies
npm install
# Build/Run
npm run dev
open http://localhost:8787
```

## Available Scripts

In `package.json`, you can find the following scripts:

```json
"scripts": {
  "clean": "rm -rf target build .wrangler dist",
  "build": "npx wrangler build",
  "dev": "npx wrangler dev",
  "deploy": "npx wrangler deploy"
}
```

- **clean**: Removes build artifacts and directories.
- **build**: Builds the project using `wrangler`.
- **dev**: Runs the project in development mode.
- **deploy**: Deploys the project to Cloudflare Workers.

## Project Structure

- **src/api/**: Contains API endpoints.
    - **say_hello.rs**: A simple server-side function for demonstration.
    - **mod.rs**: Registers server functions when the `ssr` feature is enabled.
- **src/app.rs**: Defines the main application component.
- **src/components/**: UI components.
    - **conversion_interface.rs**: Main component for TOML to JSON conversion interface.
- **src/converters/**: Conversion logic.
    - **toml2json.rs**: Converts TOML to JSON.
- **src/lib.rs**: Project library containing entry points for server-side rendering (SSR) and hydration.
- **wrangler.jsonc**: Configuration for Cloudflare Workers.

## Running the Project

### Development

To run in development mode:

```bash
npm run dev
```

### Building for Production

To build the project:

```bash
npm run build
```

### Deploying

To deploy (Requires Cloudflare login):

```bash
npm run deploy
```

## Troubleshooting
Check workers-rs Leptos project's [documentation.](https://github.com/cloudflare/workers-rs/tree/main/templates/leptos)

## Contributing

If you wish to contribute to this project:

- Fork the repository.
- Create a new branch for your feature or fix.
- Make your changes, commit them, and push to your fork.
- Open a pull request back to the original repository.

## License
This project is licensed under the MIT License.
