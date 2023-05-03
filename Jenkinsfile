@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.substratePipeline(steps: this,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$',
      pushTags: ['develop': 'dev', 'master': 'latest'],
      parachain: true,
      buildTestCmds: [
            'rm -rf ~/.cargo/registry/'
            'cargo test -r'
            'cargo build --release'
            'cp target/release/parachain-collator housekeeping/parachain-collator'
            'mv ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm ./parachain_template_runtime.compact.compressed.wasm'
      ]
)
pipeline.runPipeline()