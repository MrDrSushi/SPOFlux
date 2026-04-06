use std::{fs, collections::HashMap, time::Duration};
use anyhow::Context;
use chrono::{DateTime, Local, NaiveDate, Utc};
use colored::Colorize;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_json::{Map, Value};
use rand::{seq::IndexedRandom, rngs::ThreadRng, RngExt};
use reqwest::Client;

use crate::data::{SettingsJSON, AzureToken, UserFields};

// macro for random substituion values, it returns a value from an array using a given random number

#[macro_export]
macro_rules! random {
    // CONST &str
    ($input:expr, $seed:expr) => {
        $input.choose(&mut *$seed).copied().unwrap_or("").to_string()
    };
    // STRUCT for Airports, Countries, Locations, etc.
    ($input:expr, $seed:expr, struct) => {
        $input.choose(&mut *$seed).cloned().unwrap().clone()
    };
    // Numeric i32, i64, i128
    ($input:expr, $seed:expr, ID) => {
        //$input.choose(&mut *$seed).copied()
        $input.choose(&mut *$seed).copied()
    };
}


//
//  Date Wrappers: Min - Max - Date/Duration Formatting
//

pub fn date_min() -> DateTime<Local> {
    NaiveDate::from_ymd_opt(2000, 1, 1)     // create the date
        .unwrap()                           // safe because we know 2000-01-01 is valid
        .and_hms_opt(0, 0, 0)               // add midnight (00:00:00)
        .unwrap()
        .and_local_timezone(Local)          // attach your computer's local timezone
        .unwrap()                           // this converts it to DateTime<Local>
}

pub fn date_max() -> DateTime<Local> {
    Local::now()
}

pub fn duration_fmt(duration: std::time::Duration, mask: &str) -> String {
    let total_secs = duration.as_secs();

    let hh = format!("{:02}h", total_secs / 3600);
    let mm = format!("{:02}m", (total_secs % 3600) / 60);
    let ss = format!("{:02}s", total_secs % 60);
    let ms = format!("{:03}ms", duration.subsec_millis());

    mask.replace("hh", &hh)
        .replace("mm", &mm)
        .replace("ss", &ss)
        .replace("ms", &ms)
}

//
//  Gerneral Functions
//

pub fn truncate_to_msecs(d: Duration) -> u128 {
    d.as_secs() as u128 * 1000 + d.subsec_millis() as u128
}

pub fn format_thousands(n: i64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();

    for (i, &byte) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 && byte != b'-' {
            result.push(',');
        }
        result.push(byte as char);
    }

    result
}

pub fn parse_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error> where D: Deserializer<'de>, {
    // First, deserialize the JSON value into a String
    let s: String = Deserialize::deserialize(deserializer)?;

    // Then, attempt to parse that string into an i64 number
    s.parse::<i64>().map_err(serde::de::Error::custom)
}

pub fn generate_shipment_comment(seed: &mut ThreadRng) -> String {

    const SUBJECTS: &[&str] = &[
        "The cargo", "A suspicious crate", "The mystery box", "The pallet of doom",
        "Container #404", "The forbidden parcel", "A slightly damp envelope",
        "The heavy machinery", "A box of glitter", "The 'Do Not Open' shipment"
    ];

    const STATUS:   &[&str] = &[
        "is currently", "was found", "appears to be", "is technically",
        "is definitely not", "is allegedly", "was last seen while",
        "is persistently", "is suspiciously", "is legally"
    ];

    const ACTIONS:  &[&str] = &[
        "levitating", "glowing purple", "humming aggressively", "melting into the floor",
        "leaking neon slime", "vibrating out of phase", "whispering in French", "eating its own packaging",
        "growing small tentacles", "attracting local cats", "phasing through solid matter", "defying Newton's laws"
    ];

    const TIMING:   &[&str] = &[
        "during the full moon", "at 3:00 AM", "before the coffee was ready", "during the mandatory safety meeting",
        "while the manager wasn't looking", "in the middle of a lunch break", "right before the inspector arrived",
        "as the lights flickered", "exactly at shift change", "immediately after being touched"
    ];

    const SAFETY:   &[&str] = &[
        "Proceed with caution", "Run away immediately", "Do not make eye contact", "Wear a tinfoil hat",
        "Contact the nearest wizard", "Consult the ancient texts", "Check your insurance policy",
        "Try turning it off and on again", "Sacrifice a snack to it", "Ignore it and hope it goes away"
    ];

    const CUSTOMS:  &[&str] = &[
        "cleared by a very sleepy officer", "held for 'investigation'", "lost in the paperwork void", "bribed through with high-fives",
        "flagged for excessive weirdness", "re-routed via a dimension we don't recognize", "denied entry by a confused pigeon",
        "approved by a robot named Gary", "missing 47 necessary stamps", "subjected to a 24-hour interrogation",
        "currently being used as a paperweight in sector 7", "cleared but with a very judgmental look", "lost in the 'miscellaneous' pile",
        "accidentally exported to the moon", "confiscated by the fashion police"
    ];

    const WARNINGS: &[&str] = &[
        "contains ghosts", "highly caffeinated", "slightly glowing", "unpredictable in rain", "honestly, just be careful",
        "may contain traces of glitter", "vibrates when spoken to", "prone to spontaneous combustion", "leaks concentrated sadness",
        "attracts wild raccoons", "tastes like purple", "may cause mild hallucinations", "has its own gravitational pull",
        "is sentient on Tuesdays", "sounds like a choir of bees", "is suspiciously cold to the touch"
    ];

    const COMPLAINTS: &[&str] = &[
        "The forklift is haunted.", "I'm not paid enough for this.", "Someone ate my labeled yogurt.",
        "The warehouse cat has claimed this as a bed.", "It's too early for this.", "The printer is screaming again.",
        "The lights are flickering in Morse code.", "The breakroom microwave smells like burnt hair.", "My boots are squeaking.",
        "The roof is leaking green liquid.", "Someone replaced the water in the cooler with tea.", "The walls are sweating.",
        "I've forgotten what sunlight looks like.", "The vending machine took my last dollar.", "The radio only plays polka."
    ];

    const SOUNDS: &[&str] = &[
        "making a ticking sound", "whistling 'Despacito'", "purring loudly", "emitting a faint static noise",
        "occasionally shouting in Latin", "humming an 80s power ballad", "clucking like a nervous chicken",
        "making a sound like tearing silk", "whispering secrets about the manager", "beeping in an irregular rhythm",
        "thumping like a heartbeat", "giggling softly when moved", "echoing with the sound of distant waves"
    ];

    const SMELLS: &[&str] = &[
        "smelling faintly of wet dog", "scented like 'New Car' and regret", "smelling like a campfire in a rainstorm",
        "scented with expensive cologne and sulfur", "smelling like old library books", "smelling of ozone and ozone-adjacent things",
        "scented with fresh cinnamon and fear", "smelling like a damp basement", "scented like a very old sandwich",
        "smelling like 'The Ocean' (but the scary part)", "smelling like burnt toast and rubber"
    ];

    const CONDITIONS: &[&str] = &[
        "under a leaking roof", "in a localized gravity anomaly", "surrounded by suspicious pigeons",
        "while covered in 'Property of Area 51' stickers", "in a puddle of unknown blue liquid", "resting on a bed of hay",
        "inside a giant Ziploc bag", "while being used as a doorstop", "hidden behind a stack of empty pallets",
        "in the path of a very aggressive Roomba", "suspended by thin pieces of dental floss", "surrounded by safety cones"
    ];

    const SIGNATURE: &[&str] = &[
        "- Logged by the AI that's replacing us", "- Per the request of the Shadow Government",
        "- Sent from my Smart-Toaster", "- Dictated but not read", "- From the desk of a very tired human",
        "- Automatically generated by the Chaos Protocol", "- Verified by the Warehouse Gremlin",
        "- XOXO, Logistics Dept.", "- Written in the dark"
    ];

    const MESSAGE_TYPE: &[&str] = &[
        "Alert", "Warning", "Danger", "Attention", "Careful", "Be Aware", "Note", "Remark"
    ];

    const NAMES: &[&str] = &[
        "Al", "Alex", "Andy", "Ben", "Bill", "Bob", "Brad", "Brian", "Carl", "Charlie", "Chris", "Chuck", "Dan", "Dave", "Doug",
        "Ed", "Eric", "Frank", "Fred", "Gary", "Greg", "Hank", "Harry", "Jack", "Jake", "Jamie", "Jeff", "Jerry", "Jim", "Joe", "John",
        "Johnny", "Ken", "Kevin", "Larry", "Mark", "Matt", "Mike", "Nick", "Pat", "Paul", "Pete", "Phil", "Ray", "Rick", "Rob",
        "Ron", "Sam", "Scott", "Steve", "Ted", "Tim", "Tom", "Tony", "Vince", "Will", "Zack",
    ];


    let template_id = seed.random_range(0..5);

    let mut result = match template_id {
        0 => format!("{} {} {} {}.", random!(SUBJECTS, seed), random!(STATUS, seed), random!(ACTIONS, seed), random!(TIMING, seed)),
        1 => format!("{}: {} is {} and {}. {}!", random!(MESSAGE_TYPE, seed), random!(SUBJECTS, seed), random!(SOUNDS, seed), random!(SMELLS, seed), random!(SAFETY, seed)),
        2 => format!("{} {} {} {}.", random!(COMPLAINTS, seed), random!(SUBJECTS, seed), random!(STATUS, seed), random!(CONDITIONS, seed)),
        3 => {
            let sig = if seed.random_bool(0.5)
            {
                format!("- Signed, {} ({}) shift", random!(NAMES, seed), if seed.random_bool(0.5) { "day" } else { "night" })
            }
            else
            {
                random!(SIGNATURE, seed).to_string()
            };

            let ref_id = seed.random_range(1..10000);

            format!("Reference #{}: {} was {} {}. {}.", ref_id, random!(SUBJECTS, seed), random!(CUSTOMS, seed), random!(TIMING, seed), sig)
        },
        _ => format!("{} {} {} {}. Note: it is {}. Customs Update: {}. {}. {}. {}.", random!(SUBJECTS, seed), random!(STATUS, seed),
                     random!(ACTIONS, seed), random!(CONDITIONS, seed), random!(SOUNDS, seed), random!(CUSTOMS, seed),
                     random!(SAFETY, seed), random!(WARNINGS, seed), random!(SIGNATURE, seed))
    };

    if seed.random_bool(0.15) {
        result.push_str(&format!(" (P.S. {})", random!(COMPLAINTS, seed)));
    }

    result
}

pub fn terminate_with_message(msg: &str) -> ! {
    eprintln!("❌  {}", msg);
    std::process::exit(1);
}

pub fn load_csv_file<T>(csvfile: &str) -> Vec<T> where T: DeserializeOwned {

    let mut reader = csv::Reader::from_path(csvfile)
        .unwrap_or_else(|_| terminate_with_message(&format!("{} not found!", csvfile)));

    reader.deserialize()
        .filter_map(anyhow::Result::ok) // skip bad rows
        .collect()
}

pub fn read_json_settings(jsonfile: &str) -> SettingsJSON {

    fn get_key<'a>(obj: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
        obj.iter()
            .find(|(k, _)| k.to_lowercase() == key.to_lowercase())
            .map(|(_, v)| v)
    }

    let settings: SettingsJSON = {

        let contents = match fs::read_to_string(jsonfile) { Ok(x) => x , Err(_) => terminate_with_message("file 'settings.json' not found!") };
        let parsed: Value = serde_json::from_str(&contents).unwrap_or_else(|_| terminate_with_message("failed to parse 'settings.json' as JSON"));
        let obj = parsed.as_object().unwrap_or_else(|| terminate_with_message("JSON root is not an object"));

        SettingsJSON {
            spo_root_site: get_key(obj, "SPORootSite")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'SPORootSite'"))
                .to_string(),

            spo_site: get_key(obj, "SPOSite")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'SPOSite'"))
                .to_string(),

            spo_list: get_key(obj, "SPOList")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'SPOList'"))
                .to_string(),

            // tenant_id: get_key(obj, "tenant_id")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or_else(|| terminate_with_message("missing key 'tenant_id'"))
            //     .to_string(),

            tenant_domain: get_key(obj, "tenant_domain")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'tenant_domain'"))
                .to_string(),

            client_id: get_key(obj, "client_id")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'client_id'"))
                .to_string(),

            client_secret: get_key(obj, "client_secret")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| terminate_with_message("missing key 'client_secret'"))
                .to_string(),

            // client_thumbprint: get_key(obj, "client_thumbprint")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or_else(|| terminate_with_message("missing key 'client_thumbprint'"))
            //     .to_string(),
            //
            // entra_application_name: get_key(obj, "entra_applicationname")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or_else(|| terminate_with_message("missing key 'entra_applicationname'"))
            //     .to_string(),
            //
            // certificate_password: get_key(obj, "certificate_password")
            //     .and_then(|v| v.as_str())
            //     .unwrap_or_else(|| terminate_with_message("missing key 'certificate_password'"))
            //     .to_string(),

            soft_run: get_key(obj, "soft_run")
                .and_then(|v| v.as_bool())
                .unwrap_or_else(|| terminate_with_message("missing key 'soft_run'")),

            total_records: get_key(obj, "total_records")
                .and_then(|v| v.as_i64())
                .unwrap_or_else(|| terminate_with_message("missing key 'total_records'")),
        }
    };

    settings
}

pub async fn get_azure_token(client: &Client, client_id: &str, client_secret: &str, tenant_domain: &str) -> anyhow::Result<AzureToken> {

    #[derive(Deserialize, Debug)]
    struct TokenResponse {
        pub access_token: String,
        pub expires_in: String,
    }

    let mut token_body = HashMap::new();
    token_body.insert("grant_type",    "client_credentials");
    token_body.insert("client_id",     client_id);
    token_body.insert("client_secret", client_secret);
    token_body.insert("resource",      "https://graph.microsoft.com/");

    let token_response: TokenResponse = client
        .post(format!("https://login.microsoftonline.com/{}/oauth2/token", tenant_domain))
        .form(&token_body)
        .send()
        .await
        .context("Failed to send token request")?
        .json::<TokenResponse>()
        .await
        .context("Failed to parse token response")?;

    let expires_in_secs: i64 = token_response.expires_in.parse().context("Failed to parse expires_in as seconds")?;
    let expires_at_local     = (Utc::now() + chrono::Duration::seconds(expires_in_secs)).with_timezone(&Local);

    Ok( AzureToken { access_token : token_response.access_token , expires_datetime: expires_at_local })
}

pub async fn get_site_id(client: &Client, token: &str, spo_root_site: &str, spo_site: &str) -> anyhow::Result<String> {

    #[derive(Deserialize, Debug)]
    struct SiteIDResponse {
        id : String,   // Microsoft Graph returns the site ID as a string (e.g. "tenant.sharepoint.com,guid1,guid2")
    }

    let site: SiteIDResponse = client
        .get(format!("https://graph.microsoft.com/v1.0/sites/{}:/sites/{}", spo_root_site, spo_site))
        .bearer_auth(&token)
        .header("Content-Type", "application/json; charset=utf-8")
        .send()
        .await
        .context("Failed to request site info")?
        .json()
        .await
        .context("Failed to parse site response")?;

    Ok(site.id.splitn(2, ",").nth(1).unwrap().to_string())
}

pub async fn get_site_list_id(client: &Client, token: &str, site_id: &str, list_name: &str) -> anyhow::Result<String> {

    #[derive(Deserialize, Debug)]
    struct SiteListIDResponse {
        id : String,   // Microsoft Graph returns the site ID as a string (e.g. "tenant.sharepoint.com,guid1,guid2")
    }

    let list: SiteListIDResponse = client
        .get(format!("https://graph.microsoft.com/v1.0/sites/{}/lists/{}",site_id, list_name))
        .bearer_auth(&token)
        .header("Content-Type", "application/json; charset=utf-8")
        .send()
        .await
        .context("Failed to request site info")?
        .json()
        .await
        .context("Failed to parse site response")?;

    Ok(list.id)
}

pub async fn get_user_information_list_id(client: &Client, token: &str, site_id: &str, ) -> anyhow::Result<String> {

    #[derive(Deserialize, Debug)]
    struct ListInfo {
        id: String,
    }

    #[derive(Deserialize, Debug)]
    struct GraphResponse {
        value: Vec<ListInfo>, // This handles the "value": [ ... ] part
    }

    let response: GraphResponse = client
        .get(format!("https://graph.microsoft.com/v1.0/sites/{}/lists",site_id))
        .query(&[("$filter", "displayName eq 'User Information List'")])
        .bearer_auth(&token)
        .header("Content-Type", "application/json; charset=utf-8")
        .send()
        .await
        .context("Failed to request site info")?
        .json()
        .await
        .context("Failed to parse site response")?;

    let list_id = response.value
        .get(0)
        .map(|list| list.id.clone())
        .ok_or_else(|| anyhow::anyhow!("'User Information List' not found in the response"))?;

    Ok(list_id)
}

pub async fn get_site_users(client: &Client, token: &str, site_id: &str, user_list_id: &str) -> anyhow::Result<Vec<UserFields>> {

    //  item wrapper
    #[derive(Deserialize, Debug)]
    struct UserItem {
        fields: UserFields,
    }

    //  outer wrapper
    #[derive(Deserialize, Debug)]
    struct UserListResponse {
        value: Vec<UserItem>,
    }

    let response: UserListResponse = client
        .get(format!("https://graph.microsoft.com/v1.0/sites/{}/lists/{}/items", site_id, user_list_id))
        .bearer_auth(token)
        .query(&[("$select", "id,fields") , ("$expand", "fields($select=id,IsSiteAdmin,Deleted,SipAddress)")])
        .send()
        .await
        .context("Failed to request user items")?
        .json()
        .await
        .context("Failed to parse user items JSON")?;

    let users = response.value
        .into_iter()
        .map(|item| item.fields)
        .filter(|fields|
            {
                let is_deleted = fields.deleted.unwrap_or(false);

                let is_sip_address_na = match &fields.sip_address {
                    Some(value)  =>  value != "N/A" && !value.is_empty() ,   // !( value == "N/A" || value.is_empty() )
                    None         =>  false
                };

                !is_deleted && is_sip_address_na
            })
        .collect();

    Ok(users)
}

pub async fn send_data(client: &Client, token: &str, payload: &Value, wait_for_individual_results: bool, zeroes: usize) -> bool {
    let response = client.post("https://graph.microsoft.com/v1.0/$batch")
        .bearer_auth(token)
        .header("Content-Type", "application/json; charset=utf-8")
        .json(payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {

                if wait_for_individual_results {

                    // Parse the Batch Response

                    let json_body: Value = match res.json().await {
                        Ok(body) => body,
                        Err(_) => {
                            eprintln!("{}", "       Error - failed to parse batch response JSON.".bright_red());
                            return false;
                        }
                    };

                    let mut all_success = true;

                    // Iterate through each individual response in the batch

                    if let Some(responses) = json_body.get("responses").and_then(|r| r.as_array()) {
                        for resp in responses {

                            let id = resp.get("id").map(|v| v.to_string().replace('\"', "")).unwrap_or_else(|| "Unknown".to_string());   // Clean quotes for comparison
                            let status = resp.get("status").and_then(|s| s.as_u64()).unwrap_or(0);


                            if status == 200 || status == 201 {

                                // returned an OK, then we go ahead, no output to improve performance
                                // println!("           ✅ Record: {} - HTTP Result ({})", id, status);

                            } else {

                                // Request was usuccessful - print EACH INDIVIDUAL FAILED request!

                                all_success = false;

                                let value_id     = format!("{:0WIDTH$}" , id , WIDTH = zeroes).bright_white();
                                let value_status = format!("{:03}" , status).bright_red();

                                println!(
                                    "{lbl_id}{val_id}{lbl_status}{val_status}",
                                    lbl_id     = "       ❌ ID: ".bright_white(),
                                    val_id     = value_id,
                                    lbl_status = "       status: ".bright_white(),
                                    val_status = value_status
                                );

                                // Print the Error Message from SharePoint
                                if let Some(error_body) = resp.get("body") {
                                    println!("\n       Error details: {} \n", serde_json::to_string_pretty(error_body).unwrap_or_default());
                                }

                                // FIND and Print the exact Request Payload that failed
                                // if let Some(requests) = payload.get("requests").and_then(|r| r.as_array()) {
                                //     let failed_request = requests.iter().find(|req| { req.get("id").map(|v| v.to_string().replace('\"', "")) == Some(id.clone()) });
                                //     // We only print the "body" part to keep the console clean
                                //     if let Some(req_payload) = failed_request {
                                //         println!("   --- Failed Payload Data ---");
                                //         println!("{}", serde_json::to_string_pretty(&req_payload["body"]).unwrap_or_default());
                                //         println!("   ---------------------------\n");
                                //     }
                                // }
                            }
                        }
                    }

                    // 3. all processed, then return true
                    all_success

                } else {
                    true
                }

            } else {
                eprintln!("{} {}", "       Batch request rejected by server: {}".bright_red(), res.status());
                false
            }
        }
        Err(e) => {
            eprintln!("{} {}", "       Network/connection error: {}".bright_red(), e);
            false
        }
    }
}

/*
pub fn display_current_exe_path() {
    match std::env::current_dir() {
        Ok(path) =>  println!("   Directory:  {:?}", path.display()),
        Err(e)   => eprintln!("   Error getting current directory: {}", e),
    }

    if let Ok(exe_path) = std::env::current_exe() {
        println!("   Executable: {:?}", exe_path.display());
    }
}
*/