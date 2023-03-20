use redis::{Client, Commands, Connection};

fn main() {
    let list_name = "digits1_100";

    // Connect to Redis source-db server.
    let source_client =
        Client::open("redis://redis-12908.re-cluster1.ps-redislabs.org:12908").unwrap();
    let mut source_connection: Connection = source_client.get_connection().unwrap();

    // Insert values 1-100 into Redis list "digits1_100".
    for i in 1..=100 {
        let _: () = source_connection.rpush(list_name, i.to_string()).unwrap();
    }

    // Connect to Redis replica-db server.
    let replica_client =
        Client::open("redis://redis-12797.re-cluster1.ps-redislabs.org:12797").unwrap();
    let mut replica_connection: Connection = replica_client.get_connection().unwrap();

    // Retrieve values from Redis list "digits1_100" and print them in reverse order.
    let values: Vec<String> = replica_connection.lrange(list_name, 0, -1).unwrap();
    for value in values.into_iter().rev() {
        println!("{}", value);
    }
}