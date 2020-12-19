pipeline {
    agent {
        dockerfile true
    }
    stages {
        stage('build') {
            steps {
                sh 'cargo build'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo-test-junit --name output.xml'
                sh 'cargo test -- -Z unstable-options --format json | tee results.json
                sh 'cat results.json | cargo2junit > results.xml'
                junit result.xml
            }
        }
        stage('Doc') {
            steps {
                sh 'cargo doc'
            }
        }
    }
}
