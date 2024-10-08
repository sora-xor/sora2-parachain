@Library('jenkins-library@duty-19.09/testing_sonar_k8s_dojo') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      clippyLinter: false,
      cargoClippyTag: ':parachain',
      buildTestCmds: 'housekeeping/scripts/build.sh',
      cargoClippyCmds: ['housekeeping/scripts/clippy.sh'],
      codeCoverageCommand: './housekeeping/scripts/coverage.sh',
      pushTags: ['develop': 'dev'],
      buildArtifacts: 'sora2-parachain-runtime_rococo.compact.compressed.wasm, sora2-parachain-runtime_kusama.compact.compressed.wasm, sora2-parachain-runtime_polkadot.compact.compressed.wasm, sora2-parachain-runtime_alphanet.compact.compressed.wasm',
      sonarProjectKey: 'sora:sora2-parachain-test',
      sonarProjectName: 'sora2-parachain-test',
      dojoProductType: 'sora'
)
pipeline.runPipeline()
