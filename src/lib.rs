use lapin::{Connection, ConnectionProperties};

pub async fn connect(connection_string: &str) -> Result<lapin::Channel, lapin::Error> {
    let conn = Connection::connect(connection_string, ConnectionProperties::default()).await?;
    return Ok(conn.create_channel().await?);
}
