# Montar el docker

`$ docker run -d --name my-redis-stack -p 6379:6379  redis/redis-stack-server:latest`

docker container ls para ver el id para conectar por la shell de nuestro comando de redis
`docker exec -it 885 sh`
redis-cli
