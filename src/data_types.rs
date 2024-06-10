pub mod data_types {
    use serde::{Deserialize, Serialize};
    use strum_macros::EnumString;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ListShockersBaseResponse {
        pub message: Option<String>,
        pub data: Option<Vec<ListShockersResponse>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ListShockersResponse {
        pub shockers: Vec<ShockerResponse>,
        pub id: String,
        pub name: String,
        pub created_on: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ShockerResponse {
        pub name: Option<String>,
        pub is_paused: bool,
        pub created_on: String,
        pub id: String,
        pub rf_id: i32,
        pub model: ShockerModel,
    }

    #[derive(EnumString, Serialize, Deserialize, Debug)]
    pub enum ShockerModel {
        CaiXianlin,
        PetTrainer,
        Petrainer998DR,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ObjectBaseResponse {
        pub message: Option<String>,
        pub data: Option<String>,
    }

    #[derive(EnumString, Serialize, Deserialize, Debug)]
    pub enum ControlType {
        Stop,
        Shock,
        Vibrate,
        Sound,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ControlRequest {
        pub shocks: Vec<Shock>,
        pub custom_name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Shock {
        pub id: String,
        #[serde(rename = "type")]
        pub control_type: ControlType,
        pub intensity: u8, //min 1, max 100
        pub duration: u16, //min 300, max 30 000
        pub exclusive: bool,
    }
}
