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
                bat 'mvn clean package'  // Usamos 'bat' para ejecutar el comando en Windows
            }
        }

        stage('Run Tests') {
            steps {
                bat 'mvn test'  // Ejecuta las pruebas si las tienes configuradas
            }
        }

        stage('Build Docker Image') {
            steps {
                bat 'docker build -t %DOCKER_IMAGE% .'  // Usamos '%' para las variables de entorno en Windows
            }
        }

        stage('Push Docker Image') {
            steps {
                withDockerRegistry([credentialsId: 'docker-hub-cred', url: 'https://index.docker.io/v1/']) {
                    bat 'docker push %DOCKER_IMAGE%'  // Usamos '%' para las variables de entorno en Windows
                }
            }
        }

        stage('Deploy to Server') {
            steps {
                sshagent(['jenkins-ssh-key']) {
                    bat """
                    ssh -o StrictHostKeyChecking=no %SERVER_USER%@%SERVER_IP% << EOF
                    docker pull %DOCKER_IMAGE%
                    docker stop apinotificacions || true
                    docker rm apinotificacions || true
                    docker run -d -p 8081:8081 --name apinotificacions %DOCKER_IMAGE%
                    EOF
                    """
                }
            }
        }
    }
}
