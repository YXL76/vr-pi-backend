const { resolve } = require("path");

/**@type {import('eslint').Linter.BaseConfig}*/
module.exports = {
  extends: ["plugin:prettier/recommended"],
  overrides: [
    {
      extends: [
        "plugin:prettier/recommended",
        "prettier/@typescript-eslint",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
      ],
      files: ["**/*.ts"],
      parserOptions: {
        ecmaVersion: 10,
        project: resolve(__dirname, "tsconfig.json"),
        sourceType: "module",
      },
      plugins: ["prettier", "@typescript-eslint"],
      rules: {
        "sort-imports": [
          "warn",
          {
            allowSeparatedGroups: false,
            ignoreCase: false,
            ignoreDeclarationSort: false,
            ignoreMemberSort: false,
            memberSyntaxSortOrder: ["none", "all", "multiple", "single"],
          },
        ],
      },
    },
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    sourceType: "module",
  },
  plugins: ["prettier"],
};
