import "@ignored/hardhat-vnext-core/types/config";
import type { MochaOptions } from "mocha";
declare module "@ignored/hardhat-vnext-core/types/config" {
    interface HardhatUserConfig {
        mocha?: MochaOptions;
    }
    interface HardhatConfig {
        mocha: MochaOptions;
    }
}
//# sourceMappingURL=type-extensions.d.ts.map