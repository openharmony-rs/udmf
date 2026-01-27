## UDMF - Unified Data Management Framework on OpenHarmony

Safe Rust abstractions for the udmf API on OpenHarmony.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
udmf = "0.1.0"
```

## Development

To run tests on OpenHarmony, you need to have the OpenHarmony SDK installed and set up.
Additionally you will need to install `ohos-test-runner` and connect an OpenHarmony devices
which allows to run test executables on it. 

```sh
# Install ohos-test-runner
cargo install --locked ohos-test-runner
```

Then, you can run the tests using the following commands:

```sh
# Setup ohos-test-runner as the target runner for e.g. aarch64 OpenHarmony.
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_RUNNER=ohos-test-runner
# Tell cargo to use the linker from the OpenHarmony SDK
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER=${OHOS_SDK_NATIVE}/llvm/bin/aarch64-unknown-linux-ohos-clang
# Run cargo test (more environment variables might be needed, depending on your project)
cargo nextest run --target aarch64-unknown-linux-ohos
```
