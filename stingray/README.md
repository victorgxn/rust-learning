## Warp

- Como hacer test → https://docs.rs/warp/latest/warp/test/index.html

## Conceptos rust

## Proyecto

- Serde : JSON a objeto, objeto a JSON parecido a un json_encode() y json_decode() en php

## Postman / Redis

### Postman

- Endpoint → http://127.0.0.1:3000/alert-price || Body {JSON} : `{
    "email": "example@example.com",
    "code": "XYZ123"
} `

### Redis

- Acceder al contenedor de Redis → docker exec -it stingray redis-cli

- Listar todas las claves en Redis → keys \*

- Obtener el valor asociado a una clave específica → get example@example.com
