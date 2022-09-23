/*
 * Copyright 2008-present MongoDB, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::time::{Duration, Instant};
use std::collections::HashMap;
use string_template::Template;

use std::env;
use std::error::Error;
use std::ops::Range;
use bson::oid::ObjectId;
use bson::Document;
use rand::seq::SliceRandom;

use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use mongodb::sync::{Collection, Cursor};
use rand::Rng;
use crate::data::{*};
use crate::common::{*, add_fields, rand, TEvent};
use crate::{common, events_settlement};
use crate::data_names::NAMES_FA;

pub fn actions_land(all: &Collection<Document>, year: i32, chance: f32) -> Result<(), Box<dyn Error>>  {
    // TODO convert
    let start = Instant::now();
    let mut count = 0;
    let cursor = fetch(all, chance, doc! {
        "type": "land",
        "status": { "$ne": "settled" },
        "capacity": { "$gte": 100 },
    });
    for next in cursor {
        let land = next?;

        let mut settlement = doc! {
            "_id": ObjectId::new(),
            "xy": land.get_array("xy")?,
            "loc": land.get_array("loc")?,
            "type": "settlement",
            "name": gen_settlement_name()?,
            "capacity": land.get_i32("capacity")?,
            "description": "settlement desc",
            "population": r(12..199),
            "founded": year,
            "industry": gen_random_industry(),
            "land": refer(&land),
        };
        gen_affinity(&mut settlement, 1..3);
        gen_virtues(&mut settlement, 1..3);
        let settlement = create_record(all, settlement);

        set_fields(all, &land, doc! {
            "settlement": refer(&settlement),
            "status": "settled",
            "symbol": "X",
        });

        move_to_contents(all, &settlement, &land);


        let event: Document = doc! {
            "template_id": "founded",
            "settlement": refer(&settlement),
            "land": refer(&land),
            "year": year,
        };
        push_event(all, &event, vec![&settlement, &land]);
        count = count + 1;
    }

    println!("Settled {}: {:?}", count, start.elapsed());
    Ok(())
}

pub fn grow_populations(everything: &Collection<Document>) {
    let _ = everything.update_many(
        doc! { "type": "settlement" },
        vec![
            add_fields(doc! {
                "population": common::to_int(doc!{"$pow": ["$population", 1.005]}),
            }),
        ], None);
}

pub fn create_land(all: &Collection<Document>, xx: i32, yy: i32, rad: i32) {
    let start = Instant::now();
    // println!("Started creating land A");
    let mut vec = Vec::new();

    let across = rad * 2 + 1;
    let mut results = all.aggregate(vec![
        doc! { "$geoNear": {
            "near": loc(xx,yy),
            "distanceField": "dist",
            "maxDistance": dist(across as f32),
        }},
        doc! { "$match": { "type": "land" } },
        //doc! { "$limit":  across * across * },
    ], None).unwrap();

    let mut existing: Vec<(i32,i32)> = vec!{};
    for next in results {
        let doc = next.expect("NONE");
        let xy = get_xy(&doc);
        existing.push(xy);
    }

    let mut total = 0;

    for x in (xx-rad)..(xx+rad+1) {
        for y in (yy-rad)..(yy+rad+1) {
            let xy = (x,y);
            if existing.contains(&xy) { continue; }

            // println!("CREATING LAND AT {x},{y}");

            let mut rng = rand::thread_rng();
            let mut ox = rng.gen::<f64>() - 0.5;
            let mut oy = rng.gen::<f64>() - 0.5;

            ox = 0.0;
            oy = 0.0;

            let anum = r(1..6);
            // - ~ ^ . #
            // @ 8
            let (symbol, land_type, land_rand_name) = match anum {
                1 => ("-", "field", random_item(&[
                    "a lush meadow",
                    "golden prairies",
                ])),
                2 => ("~", "water", random_item(&[
                    "a river bend",
                    "rolling rapids",
                    "a wide river",
                    "a clear lake",
                ])),
                3 => ("^", "hills", random_item(&[
                    "rolling hills",
                    "steep cliffs",
                ])),
                4 => (".", "desert", random_item(&[
                    "an arid landscape",
                    "an oasis in the sands",
                ])),
                5 => ("#", "forest", random_item(&[
                    "a birch forest",
                    "dense woods",
                ])),
                other => ("?", "ocean", "ocean".to_string()),
            };
            if anum == 4 {
                // TODO x and y are flipped in the UI
                ox = -0.4;
            }
            if anum == 5 {
                oy = -0.2;
                ox = 0.2;
            }
            if anum == 3 {
                ox = 0.4;
            }

            let rand = r(0..1400);
            let ca:i32;
            ca = if rand <  900 { 10 }
            else if rand <  990 { 100 }
            else if rand <  999 { 1000 }
            else if rand < 1000 { 10000 }
            else { 0 };

            let rand = rng.gen::<f64>() + 0.5;
            let ca = (ca as f64 * rand) as i32;

            let land_item = random_item(&[
                "ancient stone",
                "an aquifer",

                format!("a {} {}",
                        METAL8.choose(&mut rand::thread_rng()).ok_or("").unwrap(),
                        random_item(&[
                            "sword",
                            "helmet",
                            "coin",
                            "chain",
                        ])
                ).as_str(),
                format!("a {} {}",
                        WOOD8.choose(&mut rand::thread_rng()).ok_or("").unwrap(),
                        random_item(&[
                            "toy horse",
                            "shield",
                            "statuette",
                        ])
                ).as_str(),

                "a buried traveller",
                "an unnamed warrior",
                "the remains of an ancient battle",
                "a treasure buried and forgotten",
            ]);

            //let land_items: Vec<&String> = vec1.choose_multiple(&mut rand::thread_rng(), r(1..3) as usize).collect();

            vec.push(doc! {
                "type": "land",
                "status": "undiscovered",
                "loc":  common::loc(x, y),
                "xy": [x,y],
                "name": land_rand_name,
                "land_type": land_type,
                "symbol": symbol,
                "map_offset": [ox, oy],
                "capacity": ca,
                //"contents": [],
                "grave": [land_item],
            });
        }
    }
    let total = vec.len();
    if (total > 0) {
        all.insert_many(vec, None).unwrap();
    }
    // println!("Finished creating land A: {:?}", start.elapsed());
    // println!("Lands: {}", total);

    // let start = Instant::now();
    // println!("Started creating land B");
    // let mut results2 = everything.aggregate(vec![
    //     add_fields(doc! {
    //         "type": "land",
    //         "status": "undiscovered",
    //         // "capacity": {
    //         //     "$let": {
    //         //         "vars": { "rand": randi(1000) },
    //         //         "in": {
    //         //             "$switch": { "branches": [
    //         //                 { "case": { "$lte" : [ "$$rand", 900 ] }, "then": 10 },
    //         //                 { "case": { "$lte" : [ "$$rand", 990 ] }, "then": 100 },
    //         //                 { "case": { "$lte" : [ "$$rand", 999 ] }, "then": 1000 },
    //         //                 { "case": { "$lte" : [ "$$rand", 1000 ] }, "then": 10000 },
    //         //             ]}
    //         //         }
    //         //     }
    //         // },
    //         "features": []
    //     }),
    //     doc! { "$merge": "everything" },
    // ], None)?;
    // println!("Finished creating land B: {:?}", start.elapsed()); // 3s



    // let start = Instant::now();
    // println!("Started creating land B");
    // let results2 = everything.update_many( doc!{}, vec![
    //     add_fields(doc! {
    //             "capacity": {
    //                 "$let": {
    //                     "vars": { "rand": common::randi(1000) },
    //                     "in": {
    //                         "$switch": { "branches": [
    //                             { "case": { "$lte" : [ "$$rand", 900 ] }, "then": 10 },
    //                             { "case": { "$lte" : [ "$$rand", 990 ] }, "then": 100 },
    //                             { "case": { "$lte" : [ "$$rand", 999 ] }, "then": 1000 },
    //                             { "case": { "$lte" : [ "$$rand", 1000 ] }, "then": 10000 },
    //                         ]}
    //                     }
    //                 }
    //             },
    //             // "capacity": 10,
    //             "features": [],
    //             "events": [],
    //         }),
    // ], None);
    // println!("Finished creating land B: {:?}", start.elapsed());


    // let pipeline = vec![
    //     // location: { type: "Point", coordinates: [ -73.9375, 40.8303 ] },
    //     doc! { "$documents": [ { "location": { "type": "Point", "coordinates": {  "x": 0,  "y": 0} } } ] },
    //     doc! { "$densify": { "field": "location.coordinates.x", "range": { "step": 1, "bounds": [0, land_size]} }},
    //     doc! { "$densify": { "field": "location.coordinates.y", "range": { "step": 1, "bounds": [0, land_size]} }},
    //     addFields(doc! {
    //         "_id":  random_id,
    //         "type": "land",
    //         "status": "undiscovered",
    //     }),
    //     doc! { "$merge": "everything" },
    // ];
    // Ok(())
}
