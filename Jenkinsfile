@Library('jenkins-library')

String agentLabel = 'docker-build-agent'
String registry = 'docker.soramitsu.co.jp'
String cargoAuditImage        = registry + '/build-tools/cargo_audit'
String dockerBuildToolsUserId = 'bot-build-tools-ro'
String dockerRegistryRWUserId = 'bot-sora2-rw'
String baseImageName = 'docker.soramitsu.co.jp/sora2/parachain-env:latest'
String appImageName = 'docker.soramitsu.co.jp/sora2/parachain'
String secretScannerExclusion = '.*Cargo.toml'
Boolean disableSecretScanner = false
def pushTags=['master': 'latest', 'develop': 'dev']

pipeline {
    options {
        buildDiscarder(logRotator(numToKeepStr: '20'))
        timestamps()
        disableConcurrentBuilds()
    }
    agent {
        label agentLabel
    }
    stages {
        stage('Secret scanner'){
            steps {
                script {
                    gitNotify('main-CI', 'PENDING', 'This commit is being built')
                    docker.withRegistry( 'https://' + registry, dockerBuildToolsUserId) {
                        secretScanner(disableSecretScanner, secretScannerExclusion)
                    }
                }
            }
        }
        stage('Audit') {
            steps {
                script {
                    docker.withRegistry( 'https://' + registry, dockerBuildToolsUserId) {
                        docker.image(cargoAuditImage + ':latest').inside(){
                            sh '''
                                rm -rf ~/.cargo/registry/*
                                cargo audit  > cargoAuditReport.txt || exit 0
                            '''
                            archiveArtifacts artifacts: "cargoAuditReport.txt"
                        }
                    }
                }
            }
        }   
        stage('Build & Tests') {
            steps{
                script {
                    docker.withRegistry( 'https://' + registry, dockerRegistryRWUserId) {
                        docker.image(baseImageName).inside() {
                            sh '''
                                cargo build --release
                                cp target/release/parachain-collator housekeeping/parachain-collator
                                mv ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm ./parachain_template_runtime.compact.compressed.wasm
                            '''
                            archiveArtifacts artifacts:
                                "parachain_template_runtime.compact.compressed.wasm"
                            
                        }
                    }
                }
            }
        }
        stage('Push Image') {
            when {
                expression { getPushVersion(pushTags) }
            }
            steps{
                script {
                    sh "docker build -f housekeeping/docker/release/Dockerfile -t ${appImageName} ."
                    baseImageTag = "${getPushVersion(pushTags)}"
                    docker.withRegistry( 'https://' + registry, dockerRegistryRWUserId) {
                        sh """
                            docker tag ${appImageName} ${appImageName}:${baseImageTag}
                            docker push ${appImageName}:${baseImageTag}
                        """
                    }
                }
            }
        }
    }
    post {
        always {
            script{
                gitNotify('main-CI', currentBuild.result, currentBuild.result)
            }
        }
        cleanup { cleanWs() }
    }
}