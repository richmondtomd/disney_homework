
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod home {
    use super::*;
    use super::mutual::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ApiContent {
        pub data: Data
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct Data {
        pub standard_collection: StandardCollection
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StandardCollection {
        pub call_to_action: Option<String>,
        pub collection_group: CollectionGroup,
        pub collection_id: String,
        pub containers: Vec<Container>
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CollectionGroup {
        pub collection_group_id: String,
        pub content_class: String,
        pub key: String
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Container {
        pub set: Set,
        pub r#type: String,
        pub style: String
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Set {
        pub content_class: String,
        pub items: Option<Vec<Item>>,
        pub ref_id: Option<String>,
        pub ref_id_yype: Option<String>,
        pub ref_type: Option<String>,
        pub r#type: Option<String>,
        pub text: Text
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Text {
        pub title: Title
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Title {
        pub full: Full
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Full {
        pub set: FullSet
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct FullSet{
        pub default: Default
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Default {
        pub content: Option<String>,
        pub language: Option<String>,
        pub source_entity: Option<String>
    }
}

pub mod set_ref {
    use super::*;
    use super::mutual::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RefContent {
        pub data: Data
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Data {
        #[serde(alias = "TrendingSet")]
        #[serde(alias = "CuratedSet")]
        #[serde(alias = "PersonalizedCuratedSet")]
        pub set: Option<Set>
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Set {
        content_class: Option<String>,
        experiment_token: Option<String>,
        pub items: Option<Vec<Item>>
    }

}

pub mod mutual {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Item {
        content_id: Option<String>,
        call_to_action: Option<String>,
        current_availability: Option<CurrentAvailability>, 
        encoded_series_id: Option<String>,
        episode_number: Option<String>,
        episode_sequence_number: Option<String>,
        episode_series_sequence_number: Option<String>,
        original_language: Option<String>,
        program_id: Option<String>,
        program_type: Option<String>,
        season_id: Option<String>,
        season_sequence_number: Option<String>,
        pub image: Image,
        series_id: Option<String>,
        text_experience_id: Option<String>,
        tags: Option<Vec<Tag>>,
        media_rights: Option<MediaRights>,
        ratings: Option<Vec<Rating>>,
        releases: Option<Vec<Release>>,
        r#type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CurrentAvailability {
        region: String,
        kids_mode: Option<bool>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Image {
        pub tile: Tile
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Tile {
        #[serde(rename = "1.78")]
        pub image_component: ImageComponent
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ImageComponent {
        #[serde(alias = "series")]
        #[serde(alias = "program")]
        #[serde(alias = "default")]
        pub series: Series
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Series {
        pub default: Default
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Default {
        pub master_id: String,
        master_width: usize,
        master_height: usize,
        pub url: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Tag {
        display_name: Option<String>,
        r#type: String,
        value: String 
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MediaRights {
        download_blocked: bool,
        pcon_blocked: bool
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rating {
        advisories: Vec<String>,
        description: Option<String>, 
        system: String,
        value: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Release {
        release_date: Option<String>,
        release_type: Option<String>,
        release_year: usize, 
        territory: Option<String>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediaMetadata {
        urls: Vec<String>
    }
}