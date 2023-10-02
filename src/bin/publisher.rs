use lapin::{options::*, types::FieldTable, BasicProperties};

fn main() -> Result<(), lapin::Error> {
    smol::block_on(async {
        let ch = helper::connect("amqp://localhost").await?;

        ch.queue_declare(
            "testing",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

        let publish_result = ch
            .basic_publish(
                "",
                "testing",
                BasicPublishOptions::default(),
                b"Heytayo",
                BasicProperties::default(),
            )
            .await;

        match publish_result {
            Ok(_) => println!("Message sent!"),
            Err(e) => println!("Error sending message: {:?}", e),
        }

        Ok(())
    })
}
