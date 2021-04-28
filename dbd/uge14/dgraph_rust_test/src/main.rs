use dgraph_tonic::{Mutate, Client, Mutation};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Person {
  uid: String,
  name: String,
}

#[tokio::main]
async fn main() {
  let p = Person {
    uid:  "_:alice".into(),
    name: "Alice".into(),
  };
  let mut mu = Mutation::new();
  mu.set_set_json(&p).expect("JSON");
  let client = Client::new("http://127.0.0.1:19080").expect("Dgraph client");
  let mut txn = client.new_mutated_txn();
  let response = txn.mutate(mu).await.expect("Mutated");
  txn.commit().await.expect("Transaction is commited");
}