#docker rmi -f apikey_service:latest
#docker build -t apikey_service .

read -p "Press any key to continue..." -n1 -s

docker image tag apikey_service:latest eaacrglobal101.azurecr.io/apikey_service:latest
az login

read -p "Press any key to continue... " -n1 -s

az acr login --name eaacrglobal101
docker push eaacrglobal101.azurecr.io/apikey_service:latest