pub mod client;

use async_std;

// one design is define two structs: DisConnected and Connected, but it feels wired
pub struct Client<ClientState>{

}
pub struct DisConnected{}
pub struct Connected{}
