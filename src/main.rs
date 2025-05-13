use make87_messages::core::Header;
use make87_messages::text::PlainText;
use make87_messages::well_known_types::Timestamp;
use make87_messages::CurrentTime;
use std::thread::sleep;
use std::time;

fn main() {
    make87::initialize();

    let sleep_duration = time::Duration::from_millis(1000);

    let topic_name = "OUTGOING_MESSAGE";
    match make87::resolve_topic_name(topic_name) {
        Some(topic_name) => {
            if let Some(topic) = make87::get_publisher::<PlainText>(topic_name) {
                loop {
                    let message = PlainText {
                        header: Some(Header {
                            timestamp: Timestamp::get_current_time(),
                            reference_id: 0,
                            entity_path: "/".to_string(),
                        }),
                        body: "Hello, World! 🦀".to_string(),
                    };

                    match topic.publish(&message) {
                        Ok(()) => println!("Published: {:?}", &message),
                        Err(_) => eprintln!("Failed to publish: {:?}", &message),
                    }
                    sleep(sleep_duration);
                }
            }
        }
        None => {
            panic!(
                "{}",
                format!("Failed to resolve topic name '{}'", topic_name)
            );
        }
    }
}
