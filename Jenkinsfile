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
                sh 'cargo test -- -Z unstable-options --format json | tee results.json'
                sh 'cat results.json | /root/.cargo/bin/cargo2junit > results.xml'
            }
        }
        stage('Doc') {
            steps {
                sh 'cargo doc'
            }
        }
    }
}
