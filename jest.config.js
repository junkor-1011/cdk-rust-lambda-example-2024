/** @type {import('jest').Config} */
const config = {
  testEnvironment: 'node',
  roots: ['<rootDir>/test'],
  testMatch: ['**/*.test.ts'],
  transform: {
    // '^.+\\.tsx?$': 'ts-jest',
    '^.+\\.tsx?$': [
      'esbuild-jest',
      {
        sourcemap: true,
      },
    ],
  },
  snapshotSerializers: ['<rootDir>/test/plugins/ignore-asset-hash.ts'],
};

module.exports = config;
