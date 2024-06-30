use dotenvy::dotenv;
use redis::{AsyncCommands, Client};
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

// Define una estructura para manejar la deserialización de los datos que vienen en formato JSON desde el cliente.
// `Deserialize` permite que esta estructura acepte datos en formato JSON y los convierta en una instancia de `AlertPriceRequest`.
#[derive(Deserialize)]
struct AlertPriceRequest {
    email: String,
    code: String,
}

// La macro `tokio::main` transforma la función main en una función asíncrona, permitiendo usar `await`.
#[tokio::main]
async fn main() {
    // Carga las variables de entorno desde el archivo .env.
    dotenv().ok();

    // Se obtiene la URL de Redis desde las variables de entorno.
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // Se inicializa el cliente de Redis, se envuelve en un Mutex para garantizar acceso seguro entre threads,
    // y luego se encapsula en un Arc para permitir que múltiples tareas accedan a él de manera concurrente.
    let redis_client = Client::open(redis_url).expect("No ha sido posible conectarse al servidor de redis");
    let redis_client = Arc::new(Mutex::new(redis_client));

    // Configuración de una ruta en Warp que acepta peticiones POST a '/alert-price'. Se encadena la definición de la ruta
    // con varios filtros que procesan la entrada: captura el cuerpo JSON, el cliente de Redis y define el manejador de la ruta.
    let alert_price_route = warp::post()
        .and(warp::path("alert-price"))
        .and(warp::body::json())
        .and(with_redis(redis_client.clone()))
        .and_then(handle_alert_price);

    // Inicia el servidor de Warp en el puerto 3000.
    warp::serve(alert_price_route)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

// Filtro de Warp que inyecta el cliente de Redis en la cadena de manejadores de la solicitud.
fn with_redis(
    redis_client: Arc<Mutex<Client>>,
) -> impl Filter<Extract = (Arc<Mutex<Client>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || redis_client.clone())
}

// Manejador asincrónico que procesa la solicitud POST. Utiliza el cliente de Redis para guardar información
// y retorna una respuesta que se enviará al cliente.
async fn handle_alert_price(
    payload: AlertPriceRequest,
    redis_client: Arc<Mutex<Client>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut con = redis_client
        .lock()
        .await
        .get_multiplexed_tokio_connection()
        .await
        .expect("No pudo conectarse a redis");
    let _: () = con
        .set(&payload.email, &payload.code)
        .await
        .expect("La clave no se ha podido guardar con el gmail correctamente");

    Ok(format!(
        "Alerta creada para el email {} con el id de busqueda {}",
        payload.email, payload.code
    ))
}