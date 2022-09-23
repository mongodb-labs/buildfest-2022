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
use std::thread;

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
use crate::data_names::NAMES_FA;
use crate::common::{*, add_fields, rand, TEvent};
use crate::{common, events_land, events_person, events_settlement};

pub fn generate_new_world(all: &Collection<Document>) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("Started GENERATE NEW");

    all.drop(None).unwrap();

    // location index:
    //let x = IndexModel {keys: doc!{"loc": "2d"}, options: None}; // TODO
    let mut x = mongodb::IndexModel::default();
    x.keys = doc! {"loc": "2d"};
    all.create_index(x, None).unwrap();


    let land_size = 3;

    { // initialization
        events_land::create_land(&all, 0,0,1);
        // set the initial land to a high capacity
        let r = all.update_one(doc! { "xy": { "$eq": [0,0] } }, vec![
            add_fields(doc! { "capacity": 1000,})
        ], None);
    }


    thread::spawn(|| {
        advance_years();
    });

    println!("Finished GENERATE NEW: {:?}", start.elapsed());
    Ok(())
}

fn advance_years() {
    let all = common::connect_to_everything().unwrap();
    let mut year = 10;
    for i in 0..30000 {
        year = year + 1;
        println!("Year {}", year);
        // seasons? allows multiple events to happen in one year
        for j in 0..1 {
            events_land::grow_populations(&all);
            let land_chance = 1.0 / 1.0;
            events_land::actions_land(&all, year, land_chance).unwrap();
            events_settlement::actions_settlements(&all, year).unwrap();
            events_person::actions_person(&all, year).unwrap();
        }

        let mut results = all.aggregate(vec![
            doc! {
                "$group": {
                   "_id": "$type",
                   "count": { "$count": { } }
                }
              },
        ], None).unwrap();
        println!("COUNTS OF THINGS:");
        common::print_cursor(&mut results);

    }
}
