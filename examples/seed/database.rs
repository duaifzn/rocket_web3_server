use crate::config::Config;
use crate::model::contract_model::ContractSchema;
use crate::model::user_model::UserSchema;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}
pub struct Mongo {
    pub User: Collection<UserSchema>,
    pub Contract: Collection<ContractSchema>,
}

impl Mongo {
    pub async fn connect() -> Self {
        let mongo_server = format!(
            "mongodb://{}:{}@{}/{}",
            CONFIG.mongodb_username,
            CONFIG.mongodb_password,
            CONFIG.mongodb_address,
            CONFIG.mongodb_database_name
        );
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse(mongo_server).await.unwrap();

        // Manually set an option.
        client_options.app_name = Some("MyApp".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(CONFIG.mongodb_database_name);
        //List the names of the databases in that deployment.
        // for db_name in client.list_database_names(None, None).await.unwrap() {
        //     println!("{}", db_name);
        // }
        Mongo {
            User: db.collection::<UserSchema>("users"),
            Contract: db.collection::<ContractSchema>("contracts"),
        }
    }
}
