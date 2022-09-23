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

#[macro_use]
extern crate rouille;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use bson::Document;
use bson::oid::ObjectId;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use rand::Rng;
use rand::seq::SliceRandom;
use string_template::Template;

use crate::common::add_fields;

mod common;
mod generate;
mod html;
mod data;
mod events_settlement;
mod events_land;
mod events_person;
mod data_names;


const DB_NAME: &'static str = "historygen";
const DB_URI: &'static str = "mongodb://localhost/historygen";
const SERVER: &'static str = "localhost:8080";

fn main() {
    init_gen();
    html::start_server();
}

fn init_gen() {
    let everything = common::connect_to_everything().unwrap();
    generate::generate_new_world(&everything).unwrap();

    println!("====");

    {
        let start = Instant::now();
        let unset_ignored = doc! { "$unset": ["loc", "_id", "dist"] };
        let mut results2 = everything.aggregate(vec![
            doc! { "$geoNear": {
                "near": common::loc(0,0),
                "distanceField": "dist",
            }},
            unset_ignored,
            doc! { "$match": { "type": "settlement"}},
            doc! { "$limit": 20 },
        ], None).unwrap();
        common::print_cursor(&mut results2);

        println!("Distance: {:?}", start.elapsed());
    }

    {
        let mut results = everything.aggregate(vec![
            doc! { "$count": "COUNT" },
        ], None).unwrap();
        println!("TOTAL RECORD COUNT:");
        common::print_cursor(&mut results);
    }
    {
        let mut results = everything.aggregate(vec![
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

    println!("====");
}


