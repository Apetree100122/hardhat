import { task } from "@ignored/hardhat-vnext-core/config";
const hardhatPlugin = {
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
//# sourceMappingURL=index.js.map