use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use crate::errors::{Error, Result};
use crate::config::Config;
use url::Url;

pub static STREAM_ENDPOINT: &str = "stream";
pub static WS_ENDPOINT: &str = "ws";

fn combined_stream(streams: Vec<String>) -> String { streams.join("/") }

pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>,
    conf: Config,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a, WE>
        where
            Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        Self::new_with_options(handler, Config::default())
    }

    pub fn new_with_options<Callback>(handler: Callback, conf: Config) -> WebSockets<'a, WE>
        where
            Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            conf,
        }
    }

    pub async fn connect_multiple(&mut self, endpoints: Vec<String>) -> Result<()> {
        let mut url = Url::parse(&self.conf.ws_endpoint)?;
        url.path_segments_mut()
            .map_err(|_| Error::UrlParserError(url::ParseError::RelativeUrlWithoutBase))?
            .push(STREAM_ENDPOINT);
        url.set_query(Some(&format!("streams={}", combined_stream(endpoints))));

        self.handle_connect(url).await
    }

    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.ws_endpoint, WS_ENDPOINT, endpoint);
        let url = Url::parse(&wss)?;

        self.handle_connect(url).await
    }

    async fn handle_connect(&mut self, url: Url) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}"))),
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)> { &self.socket }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some((ref mut socket, _)) = self.socket {
                // TODO: return error instead of panic?
                let message = socket.next().await.unwrap()?;

                match message {
                    Message::Text(msg) => {
                        if msg.is_empty() {
                            return Ok(());
                        }
                        let event: WE = from_str(msg.as_str())?;
                        (self.handler)(event)?;
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {e:?}")));
                    }
                }
            }
        }
        Ok(())
    }
}