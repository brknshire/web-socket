use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;

mod ws_handler;


#[tokio::main]
async fn main() {
	println!("Starting the server...");	
	
	let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
	let tx_ws = tx.clone();
    	let ws_route = warp::path("ws")
        
	.and(warp::ws())
       	.map(move |ws: warp::ws::Ws| {
            let tx = tx_ws.clone();
            ws.on_upgrade(move |websocket| ws_handler::handle_connection(websocket, tx))
        });
    	println!("Server running on ws://127.0.0.1:8080/ws");
	
	warp::serve(ws_route)
        	.run(([127, 0, 0, 1], 8080))
        	.await;
}
