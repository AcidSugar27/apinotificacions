pipeline {
    agent any

    environment {
        DOCKER_IMAGE = "acidugar27/apinotificacions"
        SERVER_IP = "159.89.191.115"
        SERVER_USER = "root"
    }

    stages {
        stage('Checkout') {
            steps {
                git branch: 'produccion', url: 'https://github.com/AcidSugar27/apinotificacions.git'
            }
        }

        stage('Build') {
            steps {
                sh 'mvn clean package'  // Ajusta esto según tu proyecto
            }
        }

        stage('Run Tests') {
            steps {
                sh 'mvn test'  // Si tienes pruebas, de lo contrario, omite
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -t $DOCKER_IMAGE .'
            }
        }

        stage('Push Docker Image') {
            steps {
                withDockerRegistry([credentialsId: 'docker-hub-cred', url: 'https://index.docker.io/v1/']) {
                    sh 'docker push $DOCKER_IMAGE'
                }
            }
        }

        stage('Deploy to Server') {
            steps {
                sshagent(['jenkins-ssh-key']) {
                    sh """
                    ssh -o StrictHostKeyChecking=no $SERVER_USER@$SERVER_IP << EOF
                    docker pull $DOCKER_IMAGE
                    docker stop apinotificacions || true
                    docker rm apinotificacions || true
                    docker run -d -p 8081:8081 --name apinotificacions $DOCKER_IMAGE
                    EOF
                    """
                }
            }
        }
    }
}
