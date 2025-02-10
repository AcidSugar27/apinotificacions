pipeline {
    agent any

    environment {
        DOCKER_IMAGE = "acidugar27/apinotificacions"
        DOCKER_TAG = "latest"
        SERVER_IP = "159.89.191.115"
        SERVER_USER = "root"
        NEXUS_URL = "http://localhost:8081/repository/docker-hosted/"
        DOCKER_CREDENTIALS = "docker-cred"  
    }

    stages {
        stage('Checkout') {
            steps {
                
                script {
                    if (env.BRANCH_NAME == 'main' || env.BRANCH_NAME == 'master' || env.BRANCH_NAME == 'develop') {
                        error "La rama ${env.BRANCH_NAME} está protegida, no se pueden realizar despliegues."
                    }
                }
                checkout scm
            }
        }

        

        stage('Build Docker Image') {
            steps {
                
                script {
                    bat "docker build -t ${DOCKER_IMAGE}:${DOCKER_TAG} ."
                }
            }
        }

        stage('Push Docker Image to Nexus') {
            when {
                branch 'produccion'  
            }
            steps {
                
                script {
                    bat "docker login -u ${NEXUS_USER} -p ${NEXUS_PASSWORD} ${NEXUS_URL}"
                    bat "docker tag ${DOCKER_IMAGE}:${DOCKER_TAG} ${NEXUS_URL}${DOCKER_IMAGE}:${DOCKER_TAG}"
                    bat "docker push ${NEXUS_URL}${DOCKER_IMAGE}:${DOCKER_TAG}"
                }
            }
        }

        stage('Deploy to DigitalOcean Server') {
            when {
                branch 'produccion'  
            }
            steps {
               
                script {
                    bat """
                    ssh -o StrictHostKeyChecking=no ${SERVER_USER}@${SERVER_IP} "
                        docker pull ${NEXUS_URL}${DOCKER_IMAGE}:${DOCKER_TAG} &&
                        docker run -d ${DOCKER_IMAGE}:${DOCKER_TAG}
                    "
                    """
                }
            }
        }
    }

    post {
        failure {
            echo 'Pipeline fallido, revisa los logs para detalles.'
        }
    }
}

