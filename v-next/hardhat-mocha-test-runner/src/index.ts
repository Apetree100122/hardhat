import type { HardhatPlugin } from "@ignored/hardhat-vnext-core/types/plugins";

import { task } from "@ignored/hardhat-vnext-core/config";

const hardhatPlugin: HardhatPlugin = {
  id: "test",
  tasks: [
    task("test", "Runs tests using the Mocha test runner")
      .addVariadicArgument({
        name: "testFiles",
        description: "An optional list of files to test",
        defaultValue: [],
      })
      .setAction(import.meta.resolve("./task-action.js"))
      .build(),
  ],
};

export default hardhatPlugin;
