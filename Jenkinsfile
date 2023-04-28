@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.parachainPipeline(steps: this,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$',
      dockerImageTags: ['develop': 'dev', 'master': 'latest']
)
pipeline.runPipeline()