use clap::Parser;
use colored::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "countryfetch")]
#[command(about = "Fetches country information from the REST Countries API", long_about = None)]
struct Args {
    #[arg()]
    country: String,
}

#[derive(Deserialize, Debug)]
struct CountryData {
    #[serde(rename = "capitalInfo")]
    capital_info: CapitalInfo,
    #[serde(rename = "name")]
    name: Name,
    #[serde(rename = "tld")]
    tld: Vec<String>,
    capital: Vec<String>,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "subregion")]
    subregion: String,
    #[serde(rename = "latlng")]
    latlng: Vec<f64>,
    #[serde(rename = "flag")]
    flag: String,
    #[serde(rename = "population")]
    population: u64,
    #[serde(rename = "timezones")]
    timezones: Vec<String>,
    #[serde(rename = "languages")]
    languages: HashMap<String, String>,
    #[serde(rename = "currencies")]
    currencies: HashMap<String, Currency>,
    #[serde(rename = "borders")]
    borders: Option<Vec<String>>,
    #[serde(rename = "continents")]
    continents: Option<Vec<String>>,
    #[serde(rename = "landlocked")]
    landlocked: bool,
    #[serde(rename = "startOfWeek")]
    start_of_week: String,
    #[serde(rename = "maps")]
    maps: Maps,
}

#[derive(Deserialize, Debug)]
struct Currency {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "symbol")]
    symbol: String,
}

#[derive(Deserialize, Debug)]
struct CapitalInfo {
    #[serde(rename = "latlng")]
    latlng: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct Name {
    #[serde(rename = "official")]
    official: String,
}

#[derive(Deserialize, Debug)]
struct Maps {
    #[serde(rename = "openStreetMaps")]
    open_street_maps: String,
}

async fn get_country_info(location: &str) -> reqwest::Result<Vec<CountryData>> {
    let url = format!(
        "https://restcountries.com/v3.1/name/{}?fields=name,capital,population,flag,region,subregion,timezones,latlng,capitalInfo,tld,languages,currencies,borders,landlocked,startOfWeek,continents,maps",
        location
    );

    let response = reqwest::get(&url).await?;
    response.json::<Vec<CountryData>>().await
}

async fn print_country_info(country: &str) {
    match get_country_info(country).await {
        Ok(data) => {
            if let Some(info) = data.get(0) {
                println!(
                    "{}: {} {}",
                    "Country".bold().blue(),
                    info.name.official,
                    info.flag
                );
                println!("{}: {:?}", "Capital".bold().blue(), info.capital.join(", "));
                println!("{}: {}", "Region".bold().blue(), info.region);
                println!("{}: {}", "Subregion".bold().blue(), info.subregion);
                println!(
                    "{}: {}/{}",
                    "LatLng".bold().blue(),
                    info.latlng[0],
                    info.latlng[1]
                );
                println!(
                    "{}: {}/{}",
                    "Capital LatLng".bold().blue(),
                    info.capital_info.latlng[0],
                    info.capital_info.latlng[1]
                );
                println!("{}: {:?}", "Timezones".bold().blue(), info.timezones);
                println!("{}: {:?}", "TLD".bold().blue(), info.tld);
                println!(
                    "{}: {}",
                    "Population".bold().blue(),
                    info.population.to_formatted_string(&Locale::en)
                );
                println!(
                    "{}: {:?}",
                    "Continent".bold().blue(),
                    info.continents
                        .as_ref()
                        .unwrap_or(&vec!["Unknown".to_string()])
                );
                println!(
                    "{}: {:?}",
                    "Languages".bold().blue(),
                    info.languages.values().collect::<Vec<&String>>()
                );
                println!(
                    "{}: {:?}",
                    "Currencies".bold().blue(),
                    info.currencies.values().collect::<Vec<&Currency>>()
                );
                println!(
                    "{}: {:?}",
                    "Borders".bold().blue(),
                    info.borders.as_ref().unwrap_or(&vec!["None".to_string()])
                );
                println!("{}: {}", "Landlocked".bold().blue(), info.landlocked);
                println!(
                    "{}: {}",
                    "Start of the week".bold().blue(),
                    info.start_of_week
                );
                println!(
                    "{}: {}",
                    "OpenStreetMap Link".bold().blue(),
                    info.maps.open_street_maps
                );
            } else {
                println!(
                    "{}",
                    "No data found for the specified country.".bold().red()
                );
            }
        }
        Err(e) => eprintln!("{}: {}", "Error fetching data".bold().red(), e),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    print_country_info(&args.country).await;
}
