use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Manufacturer {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub featured: bool,
    pub country_code: String,
    pub abbrev: String,
    pub description: String,
    pub administrator: String,
    pub founding_year: String,
    pub launchers: String,
    pub spacecraft: String,
    pub launch_library_url: String,
    pub total_launch_count: i32,
    pub consecutive_successful_launches: i32,
    pub successful_launches: i32,
    pub failed_launches: i32,
    pub pending_launches: i32,
    pub consecutive_successful_landings: i32,
    pub successful_landings: i32,
    pub failed_landings: i32,
    pub attempted_landings: i32,
    pub info_url: Option<String>,
    pub wiki_url: String,
    pub logo_url: String,
    pub image_url: String,
    pub nation_url: String,
}
