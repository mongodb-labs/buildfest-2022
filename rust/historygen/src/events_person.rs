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
use crate::events_land::create_land;


struct MovedEvent;

impl TEvent for MovedEvent {
    fn my_name(&self) -> &str { "moved" }
    fn for_type(&self) -> &str { "person" }
    fn valid_for(&self, d: &Document) -> i32 {
        let prof = d.get_str("profession").unwrap();
        if EXTERNAL_PROFESSIONS.contains(&prof) { return 0; }
        2
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let next = all.aggregate(vec![
            doc! { "$geoNear": { // SLOW
                "near": person.get_array("loc")?,
                "distanceField": "dist",
            }},
            doc! { "$match": { "type": {"$eq": "settlement"} } },
            doc! { "$match": { "xy": { "$ne": person.get_array("xy")? } } },
            doc! { "$limit": 10 },
            doc! { "$sample": { "size": 1 } },
        ], None).unwrap().next();
        if next.is_none() { return Ok(());}
        let settlement = next.expect("panic").unwrap();
        let ref_old_settlement = person.get_document("where")?;

        {
            move_to_contents(all, person, &settlement);
        }

        let event: Document = doc! {
            "template_id": "moved",
            "person": refer(&person),
            "from": ref_old_settlement,
            "to": refer(&settlement),
            "why": random_item(&[
                "to be closer to a relative",
                "to enjoy its climate",
                "to pursue business",
                "for a change of scenery",
            ]),
            "importance": 0,
            "year": year,
        };

        push_event(all, &event, vec![&person, ref_old_settlement, &settlement]);
        Ok(())
    }
}

struct LifeEvent;

impl TEvent for LifeEvent {
    fn my_name(&self) -> &str { "life" }
    fn for_type(&self) -> &str { "person" }
    fn valid_for(&self, d: &Document) -> i32 {
        5
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let event: Document = doc! {
            "template_id": "life",
            "person": refer(&person),
            "desc": random_item(&[
                "attended a feast",
                "did something interesting",
                "could not be found for several weeks",
                "was unusually happy",
            ]),
            "year": year,
        };
        push_event(all, &event, vec![&person]);
        Ok(())
    }
}

struct DiedEvent;

impl TEvent for DiedEvent {
    fn my_name(&self) -> &str { "died" }
    fn for_type(&self) -> &str { "person" }
    fn valid_for(&self, d: &Document) -> i32 {
        1 // basis event, you get one death
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let ref_old_where = person.get_document("where")?;

        // get the closest land
        let next = all.aggregate(vec![
            doc! { "$geoNear": {
                "near": person.get_array("loc")?,
                "distanceField": "dist",
            }},
            doc! { "$match": { "type": "land"} },
            doc! { "$limit": 1 },
        ], None).unwrap().next();
        if next.is_none() { return Ok(());}
        let land = next.expect("panic").unwrap();

        set_fields(all, &person, doc! {
            "death": "", // TODO
            "status": "dead",
        });
        let event: Document = doc! {
            "template_id": "died",
            "person": refer(&person),
            "where": ref_old_where,
            "land": refer(&land),
            "year": year,
            "importance": 1,
        };

        move_to_field(all, person, &land, "contents", "grave");

        push_event(all, &event, vec![&person, ref_old_where]);
        Ok(())
    }
}


pub static EXTERNAL_PROFESSIONS: &'static [&'static str] = &[
    "explorer",
];

struct CreatedEvent;

impl TEvent for CreatedEvent {
    fn my_name(&self) -> &str { "created" }
    fn for_type(&self) -> &str { "person" }
    fn valid_for(&self, d: &Document) -> i32 {
        let prof = d.get_str("profession").unwrap();
        if EXTERNAL_PROFESSIONS.contains(&prof) { return 0; }
        // if prof != "artist" {
        //     return 0;
        // }
        6
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {

        let name = gen_art_name();

        let mut item = doc! {
            "_id": ObjectId::new(),
            "type": "artwork",
            "name": name,
            "medium": random_item(&[
                "painting",
                "sculpture",
                "statue",
                "drawing",
                "tapestry",
                "embroidered garment",
            ]),
            "feeling": random_item(&[
                "sadness",
                "joy",
                "irritation",
                "peace",
                "despair",
                "fear",
                "disgust",
                "amusement",
                "worry",
                "panic",
                "horror",
                "delight",
            ]),
            "author": refer(&person),
        };
        gen_art_depictions(&mut item, 1..4);

        //gen_virtues(&mut item, 1..3);
        { // virtues not random but based on person
            let person_virtues = person.get_array("virtues").unwrap();
            let mut virtues = vec![];
            for v in person_virtues {
                let vv = v.as_str().unwrap();
                virtues.push(vv);
            }
            let chosen_virtue = virtues.choose(&mut rand::thread_rng()).unwrap().to_string();
            item.insert("virtues", vec![chosen_virtue]);
        }

        let item = create_record(all, item);

        move_to_contents(all, &item, person);

        let verb = "created";

        let event: Document = doc! {
            "template_id": "created",
            "person": refer(&person),
            "item": refer(&item),
            "verb": verb,
            "year": year,
        };
        push_event(all, &event, vec![&item, &person]);
        Ok(())
    }
}

struct ExploreEvent;

impl TEvent for ExploreEvent {
    fn my_name(&self) -> &str { "explore" }
    fn for_type(&self) -> &str { "settlement" }
    fn valid_for(&self, d: &Document) -> i32 {
        let prof = d.get_str("profession").unwrap();
        if prof == "explorer" { 20 } else { 0 }
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        // find an undiscovered location
        let next = all.aggregate(vec![
            doc! { "$geoNear": {
                "near": person.get_array("loc").unwrap(),
                "distanceField": "dist",
            }},
            doc! { "$match": { "type": "land", "status": "undiscovered", } },
            doc! { "$limit": 5 },
            doc! { "$sample": { "size": 1 } },
        ], None).unwrap().next();
        if next.is_none() { return Ok(());}
        let land = next.expect("panic").unwrap();
        let ref_old_where = person.get_document("where")?;
        // move to it
        set_fields(all, &person, doc! {
            "xy": land.get_array("xy").unwrap(),
            "loc": land.get_array("loc").unwrap(),
            "where": refer(&land),
        });
        // discover it
        set_fields(all, &land, doc! {
            "status": "discovered",
            "finder": refer(&person),
        });

        // create land around it
        {
            let (x,y) = get_xy(&land);
            create_land(all, x,y,1);
        }

        // event
        let event: Document = doc! {
            "template_id": "explore",
            "person": refer(&person),
            "from": ref_old_where,
            "to": refer(&land),
            "year": year,
            "importance": 0,
        };
        push_event(all, &event, vec![&person, ref_old_where, &land]);
        Ok(())
    }
}




struct ViewedEvent;

impl TEvent for ViewedEvent {
    fn my_name(&self) -> &str { "viewed" }
    fn for_type(&self) -> &str { "person" }
    fn valid_for(&self, d: &Document) -> i32 {
        5
    }
    fn activate(&self, all: &Collection<Document>, person: &Document, year: i32) -> Result<(), Box<dyn Error>> {
        let ref_old_where = person.get_document("where")?;

        // get the closest land
        let next = all.aggregate(vec![
            doc! { "$geoNear": {
                "near": person.get_array("loc")?,
                "distanceField": "dist",
                "maxDistance": dist(0.5),
            }},
            doc! { "$match": { "type": "artwork"} },
            doc! { "$sample": { "size": 1 } },
        ], None).unwrap().next();
        if next.is_none() { return Ok(());}

        let person_virtues = person.get_array("virtues").unwrap();

        let artwork = next.expect("panic").unwrap();
        let art_virtues = artwork.get_array("virtues").unwrap();

        let mut virtue = "";
        for v in art_virtues {
            if person_virtues.contains(v) {
                continue;
            }
            let vv = v.as_str().unwrap();
            virtue = vv;
        }

        let mut effect = "was greatly moved";

        let effect = if virtue != "" {
            let id = person.get_object_id("_id").unwrap();
            all.update_one(
                doc! { "_id": id  },
                doc! { "$push": { "virtues": virtue } },
                None).unwrap();
            format!("was inspired to greater {virtue}")
        } else {
            "was greatly moved".to_string()
        };
        let event: Document = doc! {
            "template_id": "viewed",
            "person": refer(&person),
            "artwork": refer(&artwork),
            "effect": effect,
            "year": year,
            "importance": 4,
        };

        push_event(all, &event, vec![&person, &artwork]);

        Ok(())
    }
}



pub fn actions_person(all: &Collection<Document>, year: i32) -> Result<(), Box<dyn Error>>  {
    let events: Vec<Box<dyn TEvent>> = vec![
        Box::new(LifeEvent),
        Box::new(MovedEvent),
        Box::new(DiedEvent),
        Box::new(CreatedEvent),
        Box::new(ExploreEvent),
        Box::new(ViewedEvent),
    ];
    actions_any(&all, year, &events, "person", 0.2)
}
