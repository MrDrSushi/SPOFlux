mod funcs;
mod data;

use funcs::*;
use data::*;
use rand::{seq::IndexedRandom, RngExt};
use chrono::{DateTime, Local, Duration, Utc};
use uuid::Uuid;
use anyhow::{Result};
use serde_json::{json};
use std::{time::Instant, collections::HashMap};
use colored::Colorize;
//use std::ops::Sub;

#[tokio::main]
async fn main() -> Result<()> {

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Time Measurement for feedback ]

    let timing_total_start;                                       // starting point
    let mut batch_durations: Vec<std::time::Duration> = Vec::new();   // measured time (total processing time)

    timing_total_start = Instant::now();

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Current .exe folder and environment ]

    // println!("{}", "\n▶▶ Exe Environment".bright_green());
    // display_current_exe_path();

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Program settings and configuration ]

    println!("{}", "\n▶▶ Loading Settings".green().bright_green());
    let settings = read_json_settings("settings.json");
    println!("   settings.json");

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Azure Token acquisition and SPO assets info ]

    // Step 1: Connection client
    let client = reqwest::Client::new();

    println!("{}", "\n▶▶ Requesting Azure Token".bright_green());

    // Step 2: Request Token
    let mut token = get_azure_token(&client, &settings.client_id, &settings.client_secret, &settings.tenant_domain).await?;
    let local_datetime = Local::now().format("%a, %d %b %Y %I:%M:%S %p");
    let mut token_lifetime = token.expires_datetime - Duration::minutes(5);

    println!("   Local Time:   {}", local_datetime);
    println!("   Expiring at:  {}", token.expires_datetime.format("%a, %d %b %Y %I:%M:%S %p"));
    println!("   JWT Token:    \"{}......\" \n", token.access_token.chars().take(50).collect::<String>());

    // Step 3: Get the Site ID
    println!("{}", format!("▶▶ Requesting ID from Site: {}", settings.spo_site).bright_green());
    let site_id = get_site_id(&client, &token.access_token, &settings.spo_root_site, &settings.spo_site).await?;
    println!("   Site ID: {} \n", site_id);

    // Step 4: Get the User Information List ID
    println!("{}", "▶▶ Requesting User Information List ID".bright_green());
    let user_list_id = get_user_information_list_id(&client, &token.access_token, &site_id).await?;
    println!("   User Information List ID: {} \n", user_list_id);

    // Step 5: Get the Users
    println!("{}", "▶▶ Get Site Users".bright_green());
    let users = get_site_users(&client, &token.access_token, &site_id, &user_list_id).await?;
    println!("   Users Found: {} \n", users.len());

    // Step 6: Get the ID for the target list
    println!("{}", format!("▶▶ Target List: {}/{}", settings.spo_site, settings.spo_list).bright_green());

    let list_id = get_site_list_id(&client, &token.access_token, &site_id, &settings.spo_list).await?;
    println!("   Target List ID: {} \n", list_id);

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Data Loading into transient variables ]

    println!("{}", "▶▶ Additional Data".bright_green());

    let airports: Vec<Airports> = load_csv_file("world-data-airports.csv");
    println!("   world-data-airports.csv ({} records)", airports.len());

    let countries: Vec<Countries> = load_csv_file("world-data-countries.csv");
    println!("   world-data-countries.csv ({} records)", countries.len());

    let locations: Vec<Locations> = load_csv_file("world-data-locations.csv");
    println!("   world-data-locations.csv ({} records)", locations.len());

    let ports: Vec<Ports> = load_csv_file("world-data-ports.csv");
    println!("   world-data-ports.csv ({} records)", ports.len());

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    //#region ════════════════════════════════════════════════════════════════════════════════════════[ Batch Processing ]

    // initialize the random seed

    let mut seed = rand::rng();
    let seed = &mut seed;

    // supporting fields

    let soft_run              = settings.soft_run;
    let total_records         = settings.total_records;
    let leading_zeroes        = total_records.to_string().len();
    let leading_zeroes_batch  = ((total_records / 20) as f64).ceil().to_string().len();

    let mut last_avg_request_time = std::time::Duration::new(0,0);

    let mut depends_on    = 0;
    let mut batch_counter = 0;
    let mut payload       = json!({"requests": []});

    //  Heavy collections: Locstions
    //  It is a pre-processing to save time during the loop

    let users_id_collection : Vec<i64> = users.iter().map(|u| u.id).collect();

    // Pre-build lookup table for 'Land' shipping option

    let mut locations_by_country: HashMap<String, Vec<&Locations>> = HashMap::new();

    for location in &locations {
        locations_by_country.entry(location.country.clone()).or_insert_with(Vec::new).push(location);
    }

    //
    //  Starts the loop for the creation of the records (payload creation and endpoint post request)
    //

    println!("{}","\n       ▶▶▶▶  Begining batch processing . . . \n");

    for loop_count in 1..=total_records {

        //#region ════════════════════════════════════════════════════════════════════════════════════════[ data fields ]
        //  Feeds the data into the new record

        let ts   = seed.random_range(date_min().timestamp()..=date_max().timestamp());
        let date = DateTime::from_timestamp(ts, 0).unwrap_or_else(|| Utc::now().into()).with_timezone(&Local);

        let field_item_type    : String = random!(ITEM_TYPE, seed);
        let field_item_sku     : String = Uuid::new_v4().to_string();
        let field_sector       : String = random!(SECTOR, seed);
        let field_confidential : bool   = seed.random_bool(0.50);

        let field_order_id       : i64 = seed.random_range(10_0000_000..=100_0000_000);
        let field_order_priority : String = random!(ORDER_PRIORITY, seed);
        let field_order_date     : String = format!("{}", date.format("%Y-%m-%dT%H:%M:%S")); //date.format("%d/%m/%Y %H:%M"));   // format!("{}", Local::now().format("%d/%m/%Y %H:%M"));

        let field_units_sold : i64 = seed.random_range(1..=10_000);
        let field_unit_price : f64 = (seed.random_range(1..=10_000) as f64).round() / 100.00;
        let field_unit_cost  : f64 = (field_unit_price * seed.random_range(10..=10_000) as f64 / 100.00).round() / 100.0;

        let field_total_revenue : f64 = (field_units_sold as f64 * field_unit_price).round();
        let field_total_cost    : f64 = (field_units_sold as f64 * field_unit_cost).round();
        let field_total_profit  : f64 = (field_total_revenue - field_total_cost).round();

        let field_containers    : i64 = seed.random_range( 1..=1+(field_units_sold / 1000 * 10 ));
        let field_freight_terms : String = random!(FREIGHT_TERMS, seed);
        let field_sales_channel : String = random!(SALES_CHANNEL, seed);

        let field_sales_coordinator   = random!(users_id_collection, seed, ID).unwrap().to_string();
        let field_sales_person        = random!(users_id_collection, seed, ID).unwrap().to_string();
        let field_payment_coordinator = random!(users_id_collection, seed, ID).unwrap().to_string();
        let field_shipping_foreman    = random!(users_id_collection, seed, ID).unwrap().to_string();

        let field_shipping_insured : bool = rand::random_bool(0.5);
        let field_shipping_date    : String = date.format("%Y-%m-%dT%H:%M:%S").to_string();

        let field_shipping_method = random!(SHIPPING_METHOD, seed);

        let mut field_vessel_name_or_id    : String = String::new();
        let mut field_port_of_origin       : String = String::new();
        let mut field_port_of_origin_name  : String = String::new();
        let mut field_port_of_destiny      : String = String::new();
        let mut field_port_of_destiny_name : String = String::new();

        match field_shipping_method.to_string().as_str() {
            "Air" => {
                field_vessel_name_or_id = random!(AIRLINE_NAMES, seed);

                let origin  = random!(airports, seed, struct);
                let mut destiny = random!(airports, seed, struct);

                // avoid same airport (mirrors Sea logic)
                while origin.airport_name == destiny.airport_name && airports.len() > 1 {
                    destiny = random!(airports, seed, struct);
                }

                field_port_of_origin      = format!("{}, {}", origin.municipality, origin.country);
                field_port_of_origin_name = format!("{}", origin.airport_name);

                field_port_of_destiny      = format!("{}, {}", destiny.municipality, destiny.country);
                field_port_of_destiny_name = format!("{}", destiny.airport_name);
            },
            "Land" => {
                let country = random!(countries, seed, struct);

                    if let Some(filtered_locations) = locations_by_country.get(&country.country) {

                        let origin      = random!(filtered_locations, seed, struct);
                        let mut destiny = random!(filtered_locations, seed, struct);

                        if filtered_locations.len() > 1 {
                            while origin.city == destiny.city {
                                destiny = random!(filtered_locations, seed, struct);
                            }
                        }

                        field_port_of_origin  = format!("{}, {}", origin.city, origin.country);
                        field_port_of_destiny = format!("{}, {}", destiny.city, destiny.country);
                    }
            },
            "Sea" => {
                field_vessel_name_or_id = random!(VESSEL_NAME, seed);

                let origin  = random!(ports, seed, struct);
                let mut destiny = random!(ports, seed, struct);

                while origin.port_name == destiny.port_name {
                    destiny = random!(ports, seed, struct);
                }

                field_port_of_origin      = origin.country;
                field_port_of_origin_name = origin.port_name;

                field_port_of_destiny      = destiny.country;
                field_port_of_destiny_name = destiny.port_name;
            },
            _ => {
                println!(">>>> Unknown shipping Method: {}", field_shipping_method);
            }
        };

        let field_shipping_notes = generate_shipment_comment(seed);
        let field_comments       = generate_shipment_comment(seed);
        let field_title          = format!("{}, {}", field_item_type, format_thousands(field_units_sold));

        payload["requests"].as_array_mut().unwrap().push(json!({
            "id"      : loop_count,
            "url"     : format!("sites/{}/lists/{}/items", site_id, list_id),
            "method"  : "POST",
            "headers" : { "content-type": "application/json" },
            "body"    : {
                            "fields": {
                                "ItemType": field_item_type,
                                "ItemSKU": field_item_sku,
                                "Sector": field_sector,
                                "Confidential": field_confidential,
                                "OrderID": field_order_id,
                                "OrderPriority": field_order_priority,
                                "OrderDate": field_order_date,
                                "UnitsSold": field_units_sold,
                                "UnitPrice": field_unit_price,
                                "UnitCost": field_unit_cost,
                                "TotalRevenue": field_total_revenue,
                                "TotalCost": field_total_cost,
                                "TotalProfit": field_total_profit,
                                "Containers": field_containers,
                                "FreightTerms": field_freight_terms,
                                "SalesChannel": field_sales_channel,
                                "SalesCoordinatorLookupId": field_sales_coordinator,
                                "SalesPersonLookupId": field_sales_person,
                                "PaymentCoordinatorLookupId": field_payment_coordinator,
                                "ShippingForemanLookupId": field_shipping_foreman,
                                "ShippingInsured": field_shipping_insured,
                                "ShippingDate": field_shipping_date,
                                "ShippingMethod": field_shipping_method,
                                "VesselNameOrID": field_vessel_name_or_id,
                                "PortOfOrigin": field_port_of_origin,
                                "PortOfOriginName": field_port_of_origin_name,
                                "PortOfDestiny": field_port_of_destiny,
                                "PortOfDestinyName": field_port_of_destiny_name,
                                "ShippingNotes": field_shipping_notes,
                                "Comments": field_comments,
                                "Title": field_title
                            }
            }
        }));

        //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

        depends_on += 1;

        if depends_on > 1
        {
            if let Some(last_request) = payload["requests"].as_array_mut().and_then(|x| x.last_mut()) {
                last_request.as_object_mut().unwrap().insert( "dependsOn".to_string() , json!([(loop_count - 1).to_string()]) );
            }
        }

        let current_count = payload["requests"].as_array().map(|a| a.len()).unwrap_or(0);

        if (current_count == 20) || (current_count > 0 && loop_count == settings.total_records) {

            batch_counter += 1;
            depends_on     = 0;

            let timing_request = Instant::now();

            // token refresh check

            if !soft_run {

                // checks for token expiration time

                if Local::now() >= token_lifetime {
                    println!("{}", "              ▶▶ Issuing new token ... ".bright_green());
                    token = get_azure_token(&client, &settings.client_id, &settings.client_secret, &settings.tenant_domain).await?;
                    token_lifetime = token.expires_datetime - Duration::minutes(5);
                    println!("{}", format!("              ▶▶ New token acquired, expires at {} \n", token.expires_datetime.format("%I:%M:%S %p")).bright_green());    // .format("%a, %d %b %Y %I:%M:%S %p")
                }

                // sends the requestg to the endpoint

                match send_data(&client, &token.access_token, &payload, true, leading_zeroes).await {
                    true => {
                        match serde_json::to_string_pretty(&payload) {
                            Ok(_) => {}    ,  // println!("{}\n\n", json_string),
                            Err(error_msg)  => eprintln!("{} {}", "       - Error submitting request: {}\n".bright_red(), error_msg),
                        }
                    }
                    false => {
                        eprintln!("{}", "       Error submitting request, skipping\n".bright_red())
                    },
                }
            }
            else {
                // debug/verification
                // when soft_run is enabled, we will assemble payload JSON to a csv
                // additional information from tests will be present here ...
                // to be implemented later on
            }

            let timing_request_elapsed = timing_request.elapsed();

            batch_durations.push(timing_request_elapsed);

            payload["requests"].as_array_mut().unwrap().clear();

            let timing_total_elapsed = timing_total_start.elapsed();

            let total_req_time: std::time::Duration = batch_durations.iter().sum();
            let avg_request_time = total_req_time / (batch_durations.len() as u32);

            let value_batch  = format!("{:0WIDTH$}", batch_counter, WIDTH = leading_zeroes_batch).bright_white();
            let value_tbatch = format!("{:0WIDTH$}", (total_records as f64 / 20.0).ceil() as u32, WIDTH = leading_zeroes_batch).bright_white();
            let value_req    = duration_fmt(timing_request_elapsed, "mm:ss.ms").bright_white();
            let value_time   = duration_fmt(timing_total_elapsed, "hh:mm:ss.ms").bright_white();
            let value_loop   = format!("{:0WIDTH$}", loop_count, WIDTH = leading_zeroes).bright_white();
            let value_trec   = format!("{:0WIDTH$}", total_records, WIDTH = leading_zeroes).bright_white();
            let value_avg    = duration_fmt(avg_request_time, "ss.ms").bright_white();

            let mut value_status = String::new();

            if truncate_to_msecs(avg_request_time) == truncate_to_msecs(last_avg_request_time) || last_avg_request_time == std::time::Duration::ZERO {
                //value_status = String::new();
                value_status = format!("{}","   ");
            }
            else if truncate_to_msecs(avg_request_time) > truncate_to_msecs(last_avg_request_time) {
                value_status = format!("{}","  ↗".red());
            }
            else if truncate_to_msecs(avg_request_time) < truncate_to_msecs(last_avg_request_time) {
                value_status = format!("{}","  ↙".blue());
            }

            // println!(
            //     "{lbl_batch}{val_batch}{lbl_of}{val_tbatch}{lbl_req}{val_req}{lbl_time}{val_time}{lbl_sep}{val_loop}{lbl_slash}{val_trec}{lbl_rec}{val_avg}{lbl_status}{lbl_end}",
            //     lbl_batch   = "       ════  Batch ".bright_yellow(),
            //     val_batch   = value_batch,
            //     lbl_of      = " of ".bright_yellow(),
            //     val_tbatch  = value_tbatch,
            //     lbl_req     = "          request: ".bright_yellow(),
            //     val_req     = value_req,
            //     lbl_time    = "   ──   time: ".bright_yellow(),
            //     val_time    = value_time,
            //     lbl_sep     = "   ──   ".bright_yellow(),
            //     val_loop    = value_loop,
            //     lbl_slash   = " / ".bright_yellow(),
            //     val_trec    = value_trec,
            //     lbl_rec     = " records   ──   avg: ".bright_yellow(),
            //     val_avg     = value_avg,
            //     lbl_status  = value_status,
            //     lbl_end     = " \n".bright_yellow(),
            // );

            // ── ETA Calculation ─────────────────────────────────────────────────────────────────
            let records_processed = loop_count;
            let remaining_records = total_records - records_processed;
            let remaining_batches = ((remaining_records as f64) / 20.0).ceil() as u64;

            let eta_remaining = if batch_durations.len() >= 3 {
                // Use the cumulative average batch time (very stable predictor)
                let avg_secs_f64 = avg_request_time.as_secs_f64();
                let eta_secs_f64 = avg_secs_f64 * remaining_batches as f64;
                std::time::Duration::from_secs_f64(eta_secs_f64)
            } else {
                std::time::Duration::ZERO
            };

            let value_eta = if eta_remaining == std::time::Duration::ZERO {
                "estimating ".bright_yellow()
            } else {
                duration_fmt(eta_remaining, "hh:mm:ss").bright_white()
            };

            let projected_total = if batch_durations.len() >= 3 {
                let total_batches = ((total_records as f64) / 20.0).ceil() as u64;
                let avg_secs_f64 = avg_request_time.as_secs_f64();
                std::time::Duration::from_secs_f64(avg_secs_f64 * total_batches as f64)
            } else {
                std::time::Duration::ZERO
            };

            let value_proj = duration_fmt(projected_total, "hh:mm:ss.ms").bright_white();

            println!(
                "{lbl_batch}{val_batch}{lbl_of}{val_tbatch}{lbl_req}{val_req}{lbl_time}{val_time}{lbl_sep}{val_loop}{lbl_slash}{val_trec}{lbl_rec}{val_avg}{lbl_status}{lbl_eta}{val_eta}{lbl_proj}{val_proj}{lbl_end}",
                lbl_batch   = "       ════  Batch ".bright_yellow(),
                val_batch   = value_batch,
                lbl_of      = " of ".bright_yellow(),
                val_tbatch  = value_tbatch,
                lbl_req     = "      request: ".bright_yellow(),
                val_req     = value_req,
                lbl_time    = "   ──   time: ".bright_yellow(),
                val_time    = value_time,
                lbl_sep     = "   ──   ".bright_yellow(),
                val_loop    = value_loop,
                lbl_slash   = " / ".bright_yellow(),
                val_trec    = value_trec,
                lbl_rec     = " records   ──   avg: ".bright_yellow(),
                val_avg     = value_avg,
                lbl_status  = value_status,
                lbl_eta     = "   ──   eta: ".bright_yellow(),
                val_eta     = value_eta,
                lbl_proj    = "   ──   ".bright_yellow(),
                val_proj    = value_proj,
                lbl_end     = " \n"
            );
            // ────────────────────────────────────────────────────────────────────────────────────

            last_avg_request_time = avg_request_time;
        }

    }

    // Final calculations
    let final_total_time = timing_total_start.elapsed();

    println!("{}", "\n▶▶ Execution Summary \n".bright_green());

    println!("       Records Processed:  {}", total_records);
    println!("       Batches Sent:       {}", batch_counter);
    println!("       Execution Time:     {} \n", duration_fmt(final_total_time, "hh:mm:ss.ms"));

    // Only print these if we actually sent at least one batch

    if !batch_durations.is_empty() {
        let fastest = batch_durations.iter().min().unwrap();
        let slowest = batch_durations.iter().max().unwrap();

        let total_req_time: std::time::Duration = batch_durations.iter().sum();
        let final_avg = total_req_time / (batch_durations.len() as u32);

        println!("       Fastest Request:    {} ", duration_fmt(*fastest, "ss.ms"));
        println!("       Slowest Request:    {} ", duration_fmt(*slowest, "ss.ms"));
        println!("       Average Request:    {} " , duration_fmt(final_avg, "ss.ms"));

        // Throughput overall items per second - NOT GOOD.... needs improvement
        //    let records_per_sec = ((total_records as f64) / final_total_time.as_secs_f64()).ceil();
        //    println!("       Throughput Speed: {:} records / second", records_per_sec);
    }

    println!(" ");

    //#endregion ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════

    Ok(())
}