@Library('jenkins-library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/scripts/build.sh'],
      codeCoverage: false,
      buildArtifacts: 'parachain_template_runtime.compact.compressed.wasm'
)
pipeline.runPipeline()
