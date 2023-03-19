pipeline {
    agent any
    stages {
        stage('Run') {
            agent {
                label 'builder'
            }
            stages {
                stage('Checkout') {
                    steps {
                        echo 'Checking Out..'
                    }
                }
                stage('Build') {
                    steps {
                        echo 'Building..'
                    }
                }
            }
        }
        stage('Test') {
            steps {
                echo 'Testing..'
            }
        }
        stage('Deploy') {
            steps {
                echo 'Deploying....'
            }
        }
    }
}
