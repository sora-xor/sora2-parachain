@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.substratePipeline(steps: this,
      parachain: true,
      disableSecretScanner: false,
      initSubmodules: false,
      staticScanner: false,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$|.*Jenkinsfile\$',
      dockerImageTags: ['develop': 'dev', 'master': 'latest'],
      envImageName: 'docker.soramitsu.co.jp/sora2/parachain-env:latest',
      appImageName: 'docker.soramitsu.co.jp/sora2/parachain',
      buildTestCmds: [
            'housekeeping/scripts/build.sh'
      ]
)
pipeline.runPipeline()