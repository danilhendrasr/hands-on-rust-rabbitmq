use lapin::{options::*, types::FieldTable};
use smol::stream::StreamExt;

fn main() -> Result<(), lapin::Error> {
    smol::block_on(async {
        let ch = helper::connect("amqp://localhost").await?;
        let mut consumer = ch
            .basic_consume(
                "testing",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error caught in consumer");
            let data = String::from_utf8(delivery.data.to_owned()).unwrap();
            println!("Received message: {}", data);
            delivery.ack(BasicAckOptions::default()).await?;
        }

        Ok(())
    })
}
