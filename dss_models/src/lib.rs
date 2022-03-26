
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
        pub standardCollection: StandardCollection
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StandardCollection {
        pub callToAction: Option<String>,
        pub collectionGroup: CollectionGroup,
        pub collectionId: String,
        pub containers: Vec<Container>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CollectionGroup {
        pub collectionGroupId: String,
        pub contentClass: String,
        pub key: String
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Container {
        pub set: Set,
        pub r#type: String,
        pub style: String
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Set {
        pub contentClass: String,
        pub items: Option<Vec<Item>>,
        pub refId: Option<String>,
        pub refIdType: Option<String>,
        pub refType: Option<String>,
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
    pub struct Default {
        pub content: Option<String>,
        pub language: Option<String>,
        pub sourceEntity: Option<String>
    }
}

pub mod set_ref {
    use super::*;
    use super::mutual::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RefContent {
        data: Data
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct Data {
        trendingSet: TrendingSet
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TrendingSet {
        contentClass: Option<String>,
        experimentToken: Option<String>,
        items: Vec<Item>
    }

}

pub mod mutual {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Item {
        contentId: Option<String>,
        callToAction: Option<String>,
        currentAvailability: Option<CurrentAvailability>, 
        encodedSeriesId: Option<String>,
        episodeNumber: Option<String>,
        episodeSequenceNumber: Option<String>,
        episodeSeriesSequenceNumber: Option<String>,
        originalLanguage: Option<String>,
        programId: Option<String>,
        programType: Option<String>,
        seasonId: Option<String>,
        seasonSequenceNumber: Option<String>,
        pub image: Image,
        seriesId: Option<String>,
        // text: String, //THIS IS WRONG
        textExperienceId: Option<String>,
        tags: Option<Vec<Tag>>,
        mediaRights: Option<MediaRights>,
        ratings: Option<Vec<Rating>>,
        releases: Option<Vec<Release>>,
        r#type: String,
        // videoArt: Vec<VideoArt> //ALSO WRONG
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CurrentAvailability {
        region: String,
        kidsMode: Option<bool>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Image {
        pub tile: Tile
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Tile {
        #[serde(rename = "1.78")]
        pub imageComponent: ImageComponent
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
    pub struct Default {
        pub masterId: String,
        masterWidth: usize,
        masterHeight: usize,
        pub url: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Text {

    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Tag {
        displayName: Option<String>, //assuming string
        r#type: String,
        value: String //might be string
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediaRights {
        downloadBlocked: bool,
        pconBlocked: bool
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rating {
        advisories: Vec<String>, //may need other DT
        description: Option<String>, //may need other DT
        system: String,
        value: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Release {
        releaseDate: Option<String>,
        releaseType: Option<String>,
        releaseYear: usize, //DT?
        territory: Option<String>
    }

    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct VideoArt {
    //     mediaMetadata: MediaMetadata, //check on this
    //     purpose: String
    // }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediaMetadata {
        urls: Vec<String>
    }
}