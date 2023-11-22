@Library('jenkins-library@feature/DOPS-2746/rust_sonar_dojo_slither') _

def pipeline = new org.rust.AppPipeline(steps: this,
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      cargoClippyTag: ':parachain',
      buildTestCmds: ['housekeeping/scripts/build.sh'],
      cargoClippyCmds: ['housekeeping/scripts/clippy.sh'],
      codeCoverage: true,
      codeCoverageCommand: './housekeeping/scripts/coverage.sh',
      pushTags: ['develop': 'dev'],
      buildArtifacts: 'sora2-parachain-runtime_rococo.compact.compressed.wasm, sora2-parachain-runtime_kusama.compact.compressed.wasm, sora2-parachain-runtime_polkadot.compact.compressed.wasm',
      sonarProjectKey: 'sora:sora2-parachain',
      sonarProjectName: 'sora2-parachain',
      dojoProductType: 'sora'
)
pipeline.runPipeline()
