pipeline {
    agent any

    environment {
        DOCKER_IMAGE = "acidugar27/apinotificacions"
        DOCKER_TAG = "latest"
        SERVER_IP = "159.89.191.115"
        SERVER_USER = "root"
        NEXUS_URL = "localhost:5001" 
        NEXUS_USER = "admin" 
        NEXUS_PASSWORD = "amarante" 
        DOCKER_CREDENTIALS = "docker-cred"  
        GOOGLE_CREDENTIALS_JSON = credentials('GOOGLE_CREDENTIALS_JSON')
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

        

        stage('Construir imagen de Docker') {
            steps {
                script {
                    bat "docker build --build-arg GOOGLE_CREDENTIALS_JSON=${GOOGLE_CREDENTIALS_JSON} -t ${DOCKER_IMAGE}:${DOCKER_TAG} ."
                }
            }
        }

        stage('Empujar la imagen de Docker a Nexus') {
            
            steps {
                script {
                    
                    bat "docker login -u ${NEXUS_USER} -p ${NEXUS_PASSWORD} ${NEXUS_URL}"

                    
                    bat "docker tag ${DOCKER_IMAGE}:${DOCKER_TAG} ${NEXUS_URL}/docker-hosted/${DOCKER_IMAGE}:${DOCKER_TAG}"

                    
                    bat "docker push ${NEXUS_URL}/docker-hosted/${DOCKER_IMAGE}:${DOCKER_TAG}"
                }
            }
        }

        stage('Desplegar a DigitalOcean Server') {
            
            steps {
                script {
                    
                    bat """
                    ssh -o StrictHostKeyChecking=no ${SERVER_USER}@${SERVER_IP} "
                        docker login -u ${NEXUS_USER} -p ${NEXUS_PASSWORD} ${NEXUS_URL} &&
                        docker pull ${NEXUS_URL}/docker-hosted/${DOCKER_IMAGE}:${DOCKER_TAG} &&
                        docker run -d -p 8084:8081 ${NEXUS_URL}/docker-hosted/${DOCKER_IMAGE}:${DOCKER_TAG}
                    "
                    """
                }
            }
        }
    }

    post {
        failure {
            echo 'Pipeline fallido, revisar los errores mostrados '
        }
    }
}

