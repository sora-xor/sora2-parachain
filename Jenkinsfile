@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      disableSecretScanner: false,
      initSubmodules: false,
      staticScanner: false,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$',
      dockerImageTags: ['develop': 'dev', 'master': 'latest'],
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/scripts/build.sh'],
      codeCoverageCommand: './housekeeping/scripts/coverage.sh',
      coberturaReportFile: 'cobertura_report',
      buildArtifacts: 'parachain_template_runtime.compact.compressed.wasm',
      prStatusNotif: true
)
pipeline.runPipeline()
