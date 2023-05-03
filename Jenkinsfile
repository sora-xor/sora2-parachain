@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.substratePipeline(steps: this,
      disableSecretScanner: true,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$|.*Jenkinsfile\$',
      dockerImageTags: ['develop': 'dev', 'master': 'latest'],
      parachain: true,
      staticScanner: false,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain'
      buildTestCmds: [
            'rm -rf ~/.cargo/registry/',
            'cargo test -r',
            'cargo build --release',
            'cp target/release/parachain-collator housekeeping/parachain-collator',
            'mv ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm ./parachain_template_runtime.compact.compressed.wasm'
      ]
)
pipeline.runPipeline()