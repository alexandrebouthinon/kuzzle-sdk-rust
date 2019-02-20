extern crate kuzzle_sdk;
use kuzzle_sdk::kuzzle::Kuzzle;
use kuzzle_sdk::protocols::Http;
use kuzzle_sdk::types::KuzzleOptions;
             
fn main() {
  // Instanciate your Kuzzle client                                    
  let mut kuzzle = Kuzzle::new(
      Http::new(
          KuzzleOptions::new("localhost", 7512)
      )
  );
                                                   
  // Access Kuzzle's features via its controllers 
  match kuzzle.server().now() {
      // This will display the current timestamp in Epoch millisecond format
      Ok(timestamp) => println!("{}", timestamp),
      Err(error) => eprintln!("{}", error),
  }
}
