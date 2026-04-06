use crate::parse_string_to_i64;
use serde::{Deserialize};
use chrono::{DateTime, Local};


//
//  Azure Access Token
//

#[derive(Debug)]
pub struct AzureToken {
    pub access_token: String,
    pub expires_datetime: DateTime<Local>,
}

//
//  Configuration Settings
//

pub struct SettingsJSON {
    pub spo_root_site: String,
    pub spo_site: String,
    pub spo_list: String,
    //pub tenant_id: String,
    pub tenant_domain: String,
    pub client_id: String,
    pub client_secret: String,
    //pub client_thumbprint: String,
    //pub entra_application_name: String,
    //pub certificate_password: String,
    pub soft_run: bool,
    pub total_records: i64,
}

//
//  SharePoint fields from Graph JSON response
//

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UserFields {
    #[serde(rename="id", deserialize_with="parse_string_to_i64")]
    pub id: i64,
    //pub is_site_admin: Option<bool>,
    pub deleted: Option<bool>,
    pub sip_address: Option<String>,
}

//
//  Data Strtucts
//

#[derive(Deserialize, Clone, Debug)]
pub struct Airports {
    //#[serde(rename = "AirportType")]
    //pub airport_type: String,

    #[serde(rename = "AirportName")]
    pub airport_name: String,

    /*
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "ElevationFeet")]
    pub elevation_feet: Option<i32>,

    #[serde(rename = "Continent")]
    pub continent: String,
   */

    #[serde(rename = "Country")]
    pub country: String,

    /*
    #[serde(rename = "ISO2")]
    pub iso2: String,

    #[serde(rename = "ISORegion")]
    pub iso_region: String,
    */

    #[serde(rename = "Municipality")]
    pub municipality: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Countries {
    #[serde(rename = "Country")]
    pub country: String,
}

#[derive(Deserialize, Clone)]
pub struct Locations {
    #[serde(rename = "City")]
    pub city: String,

    /*
    #[serde(rename = "CityASCII")]
    pub city_ascii: String,

    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    */

    #[serde(rename = "Country")]
    pub country: String,

    /*
    #[serde(rename = "ISO2")]
    pub iso2: String,

    #[serde(rename = "ISO3")]
    pub iso3: String,

    #[serde(rename = "AdminName")]
    pub admin_name: String,

    #[serde(rename = "Capital")]
    pub capital: String,

    #[serde(rename = "Population")]
    pub population: Option<u32>,
    */
}

#[derive(Deserialize, Clone)]
pub struct Ports {
    #[serde(rename = "PortName")]
    pub port_name: String,

    /*
    #[serde(rename = "AlternateName")]
    pub alternate_name: String,
    */

    #[serde(rename = "Country")]
    pub country: String,

    /*
    #[serde(rename = "WaterBody")]
    pub water_body: String,

    #[serde(rename = "HarborSize")]
    pub harbor_size: String,

    #[serde(rename = "HarborType")]
    pub harbor_type: String,

    #[serde(rename = "HarborUse")]
    pub harbor_use: String,

    #[serde(rename = "Railway")]
    pub railway: String,

    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    */
}


//  Not using for now - leaving for the next version

/*
pub struct ListItemRecord {
    pub field_item_type: String,
    pub field_item_sku: String,
    pub field_sector: String,
    pub field_confidential: bool,

    pub field_order_id: i32,
    pub field_order_priority: String,
    pub field_order_date: String,       //DateTime<Local>,

    pub field_units_sold: i32,
    pub field_unit_price: f64,
    pub field_unit_cost: f64,

    pub field_total_revenue: f64,
    pub field_total_cost: f64,
    pub field_total_profit: f64,

    pub field_containers: i32,
    pub field_freight_terms: String,
    pub field_sales_channel: String,

    pub field_sales_coordinator: i32,
    pub field_sales_person: i32,
    pub field_payment_coordinator: i32,
    pub field_shipping_foreman: i32,

    pub field_shipping_insured: bool,
    pub field_shipping_date: DateTime<Local>,
    pub field_shipping_method: String,

    pub field_vessel_name_or_id: String,
    pub field_port_of_origin: String,
    pub field_port_of_origin_name: String,
    pub field_port_of_destiny: String,
    pub field_port_of_destiny_name: String
}
*/

//──────────────────────────────────────────────────────────────────────────────────────────────────[ CONSTANTS ]


pub const ITEM_TYPE: &[&str] = &[
    "Air Fryer", "Airplane", "Airsoft Guns", "Alexandrite", "Aluminium",
    "Apples", "Aquamarine", "Armored Car", "Artichokes", "Avocado",
    "Axes", "Bacon", "Baggels", "Bags", "Bananas", "Batteries", "Beans",
    "Beers", "Blackberries", "Blenders", "Blowdryer", "Blueberries",
    "Boots", "Bread", "Brocoli", "Bronze", "Cabbages", "Carrots",
    "Cassette Tapes", "Chainsaws", "Charcoal", "Cherries", "Chicken",
    "Chickens", "Chocolate", "Cigarretes", "Cigars", "Coal", "Cocoa",
    "Coconut", "Coffee Beans", "Computer", "Corn", "Cosmetics", "Cows",
    "Dates", "Diamonds", "Dishwashers", "Dog Bowls", "Dolls", "Drones",
    "Dryers", "DVD Discs", "Dynamite", "Eggs", "Emeralds", "Erasers",
    "Flip flops", "Floppy Disks", "Forklifts", "Forks", "Freezers",
    "Fruits", "Gold", "Grapefruit", "Grapes", "Grenades", "Guava",
    "Hats", "High Heels", "Horses", "Ice Cream", "Insecticides", "Iron",
    "Jade", "Kiwi", "Knives", "Laptops", "Lemons", "Limes", "Lobsters",
    "Lychees", "Machine Guns", "Mangoes", "Meat", "Medical Equipment",
    "Medicine", "Microwaves", "Mint", "Missiles", "Mobile Phones",
    "Moccasins", "Motorcycles", "Mouse Traps", "Night Vision Goggles",
    "Notebooks", "Oatmeal", "Olive Oil", "Olives", "Onions", "Oolong",
    "Orange Juice", "Oranges", "Oregano", "Oysters", "Peaches",
    "Peanuts", "Pencils", "Pens", "Peppers", "Perfumes", "Pinneapples",
    "Pistols", "Pork", "Potatoes", "Prunes", "Quahog", "Quail",
    "Quail Eggs", "Quandong", "Quark", "Quartz", "Quesadilla",
    "Queso Dip", "Quiche", "Quinoa", "Radios", "Raisins", "Raspberries",
    "Recycling Material", "Refrigerators", "Revolvers", "Rice", "Rifles",
    "Roses", "Rubber", "Ruby", "Rulers", "Salmon", "Salt", "Salted Nuts",
    "Sandals", "Sapphires", "Sardines", "Sheeps", "Shoes", "Shotguns",
    "Silver", "Snickers", "Solar Panels", "Spoons", "Steak", "Steel",
    "Strawberries", "Tablets", "Tanks", "Tea Leaves", "Teddy Bears",
    "Textitles", "Tomatoes", "Topaz", "Tourmaline", "T-Shirts",
    "Tumblers", "Tuna", "Turquoise", "TVs", "Ube", "Udon", "Umbrellas",
    "Unagi", "Unsalted Nuts", "Vanilla", "Vases", "Veal",
    "Vegetable Oil", "Vegetable Soup", "Vegetables", "Velvet Beans",
    "Venison", "VHS Tapes", "Video-games", "Vienna Sausages", "Vinegar",
    "Vinyl Records", "Vodka", "Washing Machine", "Watches",
    "Water Bottles", "Watermelon", "Wine", "Wood", "Xanthan Gum",
    "X-Ray Machine", "Xylitol", "Yams", "Yeast", "Yellowfin Tuna", "Yogurt", "Zircom",
];

pub const SALES_CHANNEL: &[&str] = &["Internet", "Phone", "Sales Rep"];

pub const ORDER_PRIORITY: &[&str] = &["High", "Medium", "Low"];

pub const SHIPPING_METHOD: &[&str] = &["Air", "Sea", "Land"];

pub const SECTOR: &[&str] = &["NGO", "Public", "Private"];

pub const AIRLINE_NAMES: &[&str] = &[
    "Air France Cargo", "Alaska Air Cargo", "American Airlines Cargo",
    "American Airlines Freight", "Asiana Airlines Cargo", "Atlas Air",
    "British Airways Cargo", "British Airways World Cargo",
    "Cargo Garuda Indonesia", "Cargolux", "Caribbean Airlines",
    "Cathay Pacific Cargo", "China Airlines", "China Southern Airlines Cargo",
    "Czech Airlines Cargo", "Delta Airlines Cargo", "DHL Aviation",
    "Dragon Air Cargo", "Emirates SkyCargo", "Etihad Airways Cargo",
    "EVA Air Cargo", "FedEx Express", "Gol Transportes Aéreos",
    "Gulf Air Cargo", "Hainan Airlines Cargo", "Iberia Cargo",
    "Japan Airlines Cargo", "Kenya Airways Cargo", "KLM Cargo",
    "Korean Air Cargo", "Kuwait Airways Cargo", "LOT Polish Airlines Cargo",
    "Lufthansa Cargo", "Pakistan Intl Airlines Cargo",
    "Philippine Airlines Cargo", "Polar Air Cargo", "Qantas Freight",
    "Qatar Airways Cargo", "SAS Cargo Group", "Shenzhen Airlines Cargo",
    "Sichuan Airlines Cargo", "South African Airways", "SriLankan Cargo",
    "Sudan Airways", "Swiss WorldCargo", "Thai Airways Cargo",
    "Turkish Airlines", "Turkish Cargo", "United Airlines Cargo",
    "UPS Airlines", "Virgin Atlantic Cargo", "Virgin Australia Cargo", "WestJet Cargo",
];

pub const VESSEL_NAME: &[&str] = &[
    "Antwerpen Express", "Basle Express", "Budapest Express",
    "Cosco Belgium", "Cosco Houston", "Cosco Japan", "Cosco Oceania",
    "Cosco Pacific", "Cosco Taicang", "Cscl Bohai Sea", "Cscl Jupiter",
    "Cscl Mars", "Cscl Mercury", "Cscl Nepture", "Cscl Saturn",
    "Cscl Star", "Cscl Uranus", "Cscl Venus", "Cyprus Cape Martin",
    "Ebba Maersk", "Edith Maersk", "Eleonora Maersk", "Elly Maersk",
    "Emma Maersk", "Essen Express", "Estelle Maersk", "Eugen Maersk",
    "Evelyn Maersk", "France CMA CGM Fidelio", "Germany CMA CGM Orfeo",
    "Hamburg Express", "Hong Kong Express", "Leverkusen Express",
    "Liberia Aegiali", "Liberia As Rafaela", "Liberia Bomar Rossi",
    "Liberia Cala Paguro", "Liberia E R Felixstowe", "Liberia E R France",
    "Liberia Emirates Dana", "Liberia Emirates Wafa", "Liberia Emirates Wasl",
    "Liberia Gsl Africa", "Liberia Gsl Valerie", "Liberia Hansa Breitenburg",
    "Liberia Ikaria", "Ludwigshafen Express", "Madrid Express",
    "Malta A Idefix", "Malta Adrian Schulte", "Malta Cma Cgm Coral",
    "Marshall Islands Baltic Bridge", "Marshall Islands Baltic West",
    "Marshall Islands Cape Fawley", "Msc Filomena", "Nagoya Express",
    "New York Express", "Panama Akinada Bridge", "Panama Cosco Africa",
    "Panama Hakata Seoul", "Paris Express", "Portugal Actuaria",
    "Portugal Bernadette", "Portugal Conti Courage", "Shanghai Express",
    "Singapore Apl Columbus", "Singapore Apl Jeddah",
    "Singapore Asiatic King", "Singapore Asiatic Moon",
    "Singapore Asiatic Neptune", "Singapore Ever United",
    "Singapore Green Earth", "Singapore Green Pole", "Singapore Green Sea",
    "Singapore Interasia Heritage", "Singapore Jitra Bhum",
    "South Korea Hyundai Goodwill", "Southampton Express",
    "Thailand Jaru Bhum", "Ulsan Express", "Vienna Express",
];

pub const FREIGHT_TERMS: &[&str] = &["Prepaid", "Collect", "Elsewhere"];

//───────────────────────────────────────────────────────────────────────────────────────────────────────────────