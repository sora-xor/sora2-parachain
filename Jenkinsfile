@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      disableSecretScanner: false,
      initSubmodules: false,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$',
      dockerImageTags: ['develop': 'dev', 'master': 'latest'],
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/scripts/build.sh'],
      codeCoverage: false,
      buildArtifacts: 'parachain_template_runtime.compact.compressed.wasm',
      assignReviewers: true
)
pipeline.runPipeline()
