## Entrar al redis

- Acceder al contenedor de Redis
  docker exec -it stingray redis-cli

- Listar todas las claves en Redis
  keys \*

- Obtener el valor asociado a una clave específica
  get test@example.com
