pub mod protocol {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Command {
        pub player_id: u64,
        pub input: String, // TODO: byt mot enum
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Snapshot {
        pub tick: u32,
        pub state: String, // TODO: serialiserat world state
    }
}
