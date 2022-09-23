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
use crate::data_names::NAMES_FA;
use crate::common::{*, add_fields, rand, TEvent};
use crate::{common, events_settlement};


struct AroseEvent;

impl TEvent for AroseEvent {
    fn my_name(&self) -> &str { "arose" }
    fn for_type(&self) -> &str { "settlement" }
    fn valid_for(&self, d: &Document) -> i32 {
        10
    }
    fn activate(&self, all: &Collection<Document>, settlement: &Document, year: i32) -> Result<(), Box<dyn Error>> {


        let mut profession = random_item(&["warrior", "blacksmith", "artist", "explorer"]);


        if profession == "explorer" {
            // limit explorers;
            let cursor = all.aggregate(vec![
                doc! { "$match": { "type": "person", "profession": "explorer", "status": "alive" } },
                //doc! { "$sample": { "size": 1 } },
            ], None).unwrap();

            let mut count = 0;
            for next in cursor {
                count = count + 1;
            }
            if (count > 5) {
                profession = "artist".to_string();
            }

        }

        let mut person = doc! {
            "_id": ObjectId::new(),
            "type": "person",
            "name": gen_person_name(),
            "profession": profession,
            //"settlement": refer(&settlement),
            "where": refer(&settlement),
            "status": "alive",
            "death": "",
            "contents": []
        };
        move_to(settlement, &mut person);

        gen_affinity(&mut person, 1..4);
        gen_virtues(&mut person, 1..2);
        gen_dislikes(&mut person, 1..4);
        gen_appearance(&mut person, 2..3);
        let person = create_record(all, person);

        let event: Document = doc! {
            "template_id": "arose",
            "person": refer(&person),
            "birthplace": refer(&settlement),
            "year": year,
            "importance": 2,
        };
        push_event(all, &event, vec![&person, &settlement]);
        Ok(())
    }
}


struct FamineEvent;

impl TEvent for FamineEvent {
    fn my_name(&self) -> &str { "famine" }
    fn for_type(&self) -> &str { "settlement" }
    fn valid_for(&self, d: &Document) -> i32 {
        let cap = d.get_i32("capacity").unwrap();
        let pop = d.get_i32("population").unwrap();
        if pop > cap { 100 } else { 0 }
    }
    fn activate(&self, all: &Collection<Document>, settlement: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let pop = settlement.get_i32("population")?;
        let deaths = (pop / r(2..5)) + 2; // plural
        let pop = pop - deaths;
        let event: Document = doc! {
            "template_id": "famine",
            "deaths": deaths,
            "cause": common::random_item(&[
                "a famine struck",
                "a plague struck",
                "a heat wave struck",
                "a cold snap struck",
                "a flood struck",
                "a fire struck",
            ]),
            "settlement": refer(&settlement),
            "year": year,
            "importance": 3,
        };
        set_fields(all, &settlement, doc! {
            "settlement": refer(&settlement),
            "population": pop,
        });
        push_event(all, &event, vec![&settlement]);
        Ok(())
    }
}


struct FeastEvent;

impl TEvent for FeastEvent {
    fn my_name(&self) -> &str { "feast" }
    fn for_type(&self) -> &str { "settlement" }
    fn valid_for(&self, d: &Document) -> i32 {
        20
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let event: Document = doc! {
            "template_id": "feast",
            "settlement": refer(&person),
            "desc": random_item(&[
                "celebrated a great feast",
                "built a public building",
                "experienced unusual weather",
                "commemorated a great hero",
                "sent a gift to a nearby settlement",
            ]),
            "year": year,
        };
        push_event(all, &event, vec![&person]);
        Ok(())
    }
}

pub fn actions_settlements(all: &Collection<Document>, year: i32) -> Result<(), Box<dyn Error>>  {
    let events: Vec<Box<dyn TEvent>> = vec![
        Box::new(AroseEvent),
        Box::new(FamineEvent),
        Box::new(FeastEvent),
    ];
    actions_any(&all, year, &events, "settlement", 0.2)
}
