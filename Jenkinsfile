@Library('jenkins-library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: ['housekeeping/scripts/build.sh'],
      codeCoverage: false,
      buildArtifacts: 'sora2-parachain-runtime_rococo.compressed.wasm sora2-parachain-runtime_kusama.compressed.wasm sora2-parachain-runtime_polkadot.compressed.wasm'
)
pipeline.runPipeline()
