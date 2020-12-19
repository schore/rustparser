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
                junit output.xml
            }
        }
        stage('Doc') {
            steps {
                sh 'cargo doc'
            }
        }
    }
}
