const coverageToNumber = 50; // [0..100]

/*
 * For a detailed explanation regarding each configuration property and type check, visit:
 * https://jestjs.io/docs/configuration
 */

export default {
	testTimeout: 600 * 1000,
	setTimeout: 600 * 1000,
	verbose: true,
	rootDir: "./",
	clearMocks: true, // clear mocks before every test
	resetMocks: false, // reset mock state before every test
	testMatch: [
		// '<rootDir>/src/**/*.spec.ts', // Commenting cache test for github actions
		"<rootDir>/**/*.test.ts",
		"<rootDir>/**/*.test.js",
		"<rootDir>/**/*.spec.ts",
		"<rootDir>/**/*.spec.ts",
	], // match only tests inside /tests folder
	testPathIgnorePatterns: ["<rootDir>/node_modules/"], // exclude unnecessary folders

	// following lines are about coverage
	collectCoverage: true,
	collectCoverageFrom: ["<rootDir>/src/**/*.ts", "<rootDir>/src/**/*.js"],
	coverageDirectory: "<rootDir>/coverage",
	coverageReporters: ["lcov"],
};
