#![feature(rustc_private)]
#[macro_use]
extern crate serde_derive ;
extern crate serde_json ;
extern crate reqwest;
#[macro_use] extern crate prettytable;
extern crate clap;

use clap::{Arg, App};
use prettytable::{Table};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GeneratedType {
    params: Params,
    entries: Vec<Entry>,
    #[serde(rename = "driversResult")]
    drivers_result: Vec<DriversResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Params {
    #[serde(rename = "eventName")]
    event_name: String,
    #[serde(rename = "sessionId")]
    session_id: i64,
    timestamp: String,
    #[serde(rename = "elapsedTime")]
    elapsed_time: String,
    racestate: String,
    safetycar: String,
    weather: String,
    #[serde(rename = "airTemp")]
    air_temp: String,
    #[serde(rename = "trackTemp")]
    track_temp: String,
    humidity: i64,
    pressure: f64,
    #[serde(rename = "windSpeed")]
    wind_speed: f64,
    #[serde(rename = "windDirection")]
    wind_direction: i64,
    elapsed: i64,
    remaining: i64,
    svg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Entry {
    ranking: i64,
    number: i64,
    id: i64,
    driver_id: i64,
    state: String,
    #[serde(rename = "isWEC")]
    is_wec: bool,
    category: String,
    nationality: String,
    team: String,
    tyre: String,
    driver: String,
    car: String,
    lap: String,
    gap: String,
    #[serde(rename = "gapPrev")]
    gap_prev: String,
    #[serde(rename = "classGap")]
    class_gap: String,
    #[serde(rename = "classGapPrev")]
    class_gap_prev: String,
    lastlap: String,
    #[serde(rename = "lastLapDiff")]
    last_lap_diff: String,
    pitstop: i64,
    bestlap: String,
    speed: String,
    #[serde(rename = "bestSector1")]
    best_sector1: String,
    #[serde(rename = "bestSector2")]
    best_sector2: String,
    #[serde(rename = "bestSector3")]
    best_sector3: String,
    #[serde(rename = "currentSector1")]
    current_sector1: String,
    #[serde(rename = "currentSector2")]
    current_sector2: String,
    #[serde(rename = "currentSector3")]
    current_sector3: String,
    sector: i64,
    #[serde(rename = "lastPassingTime")]
    last_passing_time: String,
    #[serde(rename = "categoryPosition")]
    category_position: i64,
    position: Position,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Position {
    percent: f64,
    sector: i64,
    timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DriversResult {
    #[serde(rename = "driverID")]
    driver_id: i64,
    laps: i64,
    #[serde(rename = "bestLap")]
    best_lap: String,
    #[serde(rename = "lastLap")]
    last_lap: String,
    #[serde(rename = "drivingTime")]
    driving_time: i64,
    #[serde(rename = "percentDriving")]
    percent_driving: i64,
    #[serde(rename = "bestLapNumber")]
    best_lap_number: i64,
    #[serde(rename = "lastLapDiff")]
    last_lap_diff: String,
    pitstop: i64,
    #[serde(rename = "categoryPosition")]
    category_position: Option<i64>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("ttimes")
        .version("0.1")
        .author("Michal Skalski <michal@skalski.org>")
        .about("Display time table of WEC race")
        .arg(Arg::with_name("categories")
                 .short("c")
                 .long("categories")
                 .takes_value(true)
                 .help(
           "Takes comma separated list of WEC car categories to display"))
        .arg(Arg::with_name("url")
                 .short("u")
                 .long("url")
                 .takes_value(true)
                 .help(
           "Url for race json data"))
        .get_matches();
    let categories = matches.value_of("categories").unwrap_or("");
    let mut category_list = Vec::new();
    if ! categories.is_empty() {
       category_list = categories.split(',').collect();
    }
    let url = matches.value_of("url").unwrap_or(
      "https://storage.googleapis.com/fiawec-prod/assets/live/WEC/__data.json");

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    let client = reqwest::Client::new();
    let mut resp = client.get(url)
    .query(&[("_", in_ms.to_string())])
    .send()?;
    let wec: GeneratedType = match resp.json() {
       Ok(r) => r,
       Err(e) => panic!("Could not read race data. Error was:\n {:?}", e),
    };

    let mut status = format!(
        "{} - Status: {} -  Elapsed time: {}", wec.params.event_name,
                                               wec.params.racestate,
                                               wec.params.elapsed_time);
    if wec.params.safetycar == "true" {
        status = format!("{} Safety Car!", status);
    }

    println!("{}", status);

    let mut table = Table::new();
    table.add_row(row!["Pos",
                       "Num",
                       "Cat",
                       "PiC",
                       "Driver/Team",
                       "State",
                       "Laps",
                       "Lap time",
                       "Int",
                       "PIT"]);
    for entry in &wec.entries {
      if category_list.is_empty() || category_list.contains(&entry.category.as_str()) {
          table.add_row(row![entry.ranking,
                             entry.number,
                             entry.category,
                             entry.category_position,
                             format!("{}/{}", entry.driver, entry.team),
                             entry.state,
                             entry.lap,
                             entry.lastlap,
                             entry.class_gap,
                             entry.pitstop
                             ]);
      } else {
          continue;
      }
    }
    table.printstd();
    Ok(())
}
