#rm -rf target/
docker rmi -f apikey_service:latest
docker build -t apikey_service .
