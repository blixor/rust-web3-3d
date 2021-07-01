// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  alias: {
      '@wasm': './wasm/pkg',
      },
  mount: {
      "frontend": "/",
  },
  plugins: [
    [
      'snowpack-plugin-wasm-pack',
      {
        projectPath: './wasm',
      },
    ],
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
};
