use warp::ws::{WebSocket, Message};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use futures_util::{StreamExt, SinkExt};
 
pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
	
	println!("Websocket connection started!");
	
	let (mut ws_sender, mut ws_receiver) = ws.split();
    	
	let mut rx = tx.lock().await.subscribe();
	
	tokio::spawn(async move {
		while let Ok(msg) = rx.recv().await {
            		if ws_sender.send(Message::text(msg)).await.is_err() {
                		break;
            		}
        	}
   	});
    		while let Some(result) = ws_receiver.next().await {
        		match result {
            			Ok(message) => {
					if let Ok(text) = message.to_str() {
                    				tx.lock().await.send(text.to_string()).expect("Failed to broadcast message");
                			}
            			},
           			Err(e) => {
					println!("Failed to lock the mutex: {:?}", e);
					return;
				}
        		}
    		}
	}
