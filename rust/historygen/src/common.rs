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

use std::any::Any;
use std::collections::HashMap;
use std::{env, thread};
use std::error::Error;
use std::time::{Duration, Instant};

use bson::{Array, Document};
use bson::oid::ObjectId;
use chrono::{TimeZone, Utc};
use mongodb::sync::{Client, Collection, Cursor};
use mongodb::bson::doc;
use rand::Rng;
use rand::seq::SliceRandom;
use string_template::Template;
use std::ops::Range;
use rand::prelude::ThreadRng;
// use serde_json::{Map, Value};
use crate::{common, DB_NAME, DB_URI};
use crate::data::{CLOTH4, CROP32, LIVESTOCK16, METAL8, STONE8, VIRTUES16, WOOD8};
use crate::data_names::NAMES_FA;

pub fn connect_to_everything() -> Result<Collection<Document>, Box<dyn Error>>   {
    let client = Client::with_uri_str(DB_URI)?;
    let db = client.database(DB_NAME);
    let everything: Collection<Document> = db.collection("everything");
    Ok(everything)
}

pub fn add_fields(fields: Document) -> Document {
    doc! { "$addFields":  fields}
}

pub fn to_int(i: Document) -> Document {
    doc! { "$toInt": i }
}

pub fn randi(i: i32) -> Document {
    doc! { "$multiply": [ rand(), i ] }
}

pub fn multiply(a: Document, b: Document) -> Document {
    doc! { "$multiply": [ a, b ] }
}

pub fn multiply3(a: Document, b: Document, c: Document) -> Document {
    doc! { "$multiply": [ a, b, c ] }
}

pub fn subtract(a: Document, b: Document) -> Document {
    doc! { "$subtract": [ a, b ] }
}

pub fn abs(b: Document) -> Document {
    doc! { "$abs": b }
}

pub fn literal(b: i32) -> Document {
    doc! { "$literal": b }
}

pub fn literal_f32(b: f32) -> Document {
    doc! { "$literal": b }
}

pub fn difference(b: Document, c: Document) -> Document {
    abs(subtract(b, c))
}

pub fn rand() -> Document {
    doc! { "$rand": {} }
}

pub fn loc(x: i32, y: i32) -> Vec<f32> {
    vec![x as f32 / 10000.0, y as f32 / 10000.0]
}

pub fn dist(x: f32) -> f32 {
    x as f32 / 10000.0
}

pub fn print_cursor(results: &mut Cursor<Document>) {
    let mut i = 0;
    for next in results {
        let doc = next.expect("NONE");
        println!(">> {}", doc);
        i = i + 1;
    }
    println!(">> ({} results)", i);
}

pub fn print_databases(client: Client) -> Result<(), Box<dyn Error>> {
    println!("DATABASES:");
    for name in client.list_database_names(None, None) {
        for x in name {
            println!("- {}", x);
        }
    }
    Ok(())
}

pub fn r(range: Range<i32>) -> i32 {
    rand::thread_rng().gen_range(range)
}

pub fn fetch(all: &Collection<Document>, chance: f32, matches: Document) -> Cursor<Document> {
    let mut cursor = all.aggregate(vec![
        doc! { "$match": matches},
        doc! { "$match": {"$expr":  {"$lte": [rand(), literal_f32(chance)]}  }}
    ], None).unwrap();
    cursor
}

pub fn create_record(everything: &Collection<Document>, settlement: Document) -> Document {
    everything.insert_one(&settlement, None).unwrap();
    settlement
}

pub fn set_fields(everything: &Collection<Document>, target: &Document, fields: Document) {
    everything.update_many(
        doc! { "_id": target.get_object_id("_id").unwrap() },
        vec![ add_fields(fields, )],
        None).unwrap();
}

pub(crate) fn refer(settlement: &Document) -> Document {
    let settlement_ref = doc! {
            "_id": settlement.get_object_id("_id").unwrap(),
            "name": settlement.get_str("name").unwrap(),
        };
    settlement_ref
}

pub fn push_event(all: &Collection<Document>, event: &Document, p2: Vec<&Document>) {

    // let start2 = Instant::now();

    let ids: Vec<ObjectId> = p2.into_iter().map(|x:&Document| x.get_object_id("_id").unwrap()).collect();
    // let event = event.clone();
    // thread::spawn(|| {
    //     let all = connect_to_everything().unwrap();
    // });

    // for x in ids {
    //     all.update_one(
    //         doc! { "_id": x  },
    //         doc! { "$push": { "events": &event } },
    //         None).unwrap();
    // }

    all.update_many(
        doc! { "_id": {"$in": ids}  },
        doc! { "$push": { "events": event } },
        None).unwrap();

    // if start2.elapsed().as_millis() > 10 {
    //     println!("------------------------SLOW PUSH {:?} ",
    //              start2.elapsed());
    // }
}

pub trait TEvent {
    fn my_name(&self) -> &str;
    fn for_type(&self) -> &str;
    fn valid_for(&self, d: &Document) -> i32;
    fn activate(&self, all: &Collection<Document>, d: &Document, year: i32) -> Result<(), Box<dyn Error>> ;
}

pub(crate) fn actions_any(all: &&Collection<Document>, year: i32, events: &Vec<Box<dyn TEvent>>, dtype: &str, chance: f32) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut count = 0;
    let cursor = fetch(all, chance, doc! {
        "type": dtype,
        "status": {"$ne": "dead"},
    });
    for next in cursor {
        let target = next?;
        let mut valid_events: Vec<&Box<dyn TEvent>> = Vec::new();
        let mut total = 0;
        for event in events {
            let pref = event.valid_for(&target);
            if pref == 0 {
                continue;
            }
            total = total + pref;
            valid_events.push(event);
        }
        let mut total = r(0..total);
        for event in valid_events {
            let pref = event.valid_for(&target);
            if total < pref {
                let start2 = Instant::now();
                //println!{"event start activate {:?}", event.my_name()}
                event.activate(&all, &target, year).unwrap();
                if start2.elapsed().as_millis() > 100 {
                    println!("------------------------SLOW event activate {:?} {:?}",
                             start2.elapsed(), event.my_name());
                }
                count = count + 1;
                break;
            } else {
                total = total - pref;
            }
        }

        // let event = valid_events.choose(&mut rand::thread_rng()).unwrap();
    }
    println!("{} turn {}: {:?}", dtype, count, start.elapsed());
    Ok(())
}


pub fn gen_person_name<>() -> String  {
    random_item(NAMES_FA)
}

pub fn random_item(x: &[&str]) -> String {
    x.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn random_item_v(x: Vec<&str>) -> String {
    x.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn gen_settlement_name<>() -> Result<String, Box<dyn Error>>  {
    let n1 = vec![
        "Raven", "Snake", "Bear", "Dove", "Goat", "Wren",
        "Smoke", "Mist", "Dust",
        "Pine", "Rose",
        "Sunshine",
        "Crimson", "Scarlet", "Violet", "Indigo", "Auburn",
    ];
    let n2 = vec![
        "Hold", "Keep",
        "Hollow", "Deep", "Burrow",
        "Stow",
        //"Retreat", "Den",

        "Point", "Pass",
        "View",
        "Side", "Heights", "Hill", "Mount", "Ridge", "Hurst",
        "Dell", "Dale", "Vale", "Valley", "Glen",
        "Lake", "Wood",
        "Clear",
        "Meadow",
    ];
    let name = format!("{} {}",
                       n1.choose(&mut rand::thread_rng()).ok_or("")?,
                       n2.choose(&mut rand::thread_rng()).ok_or("")?);
    Ok(name)
}

// pub fn json_stuff() -> Document {
//     let my_str = include_str!("terms.json");
//     // println!("{my_str}");
//     let value: Map<String, Value> = serde_json::from_str(my_str).expect("p");
//     let document = Document::try_from(value).expect("p");
//     // println!("{:?}", document);
//     document
// }

pub fn gen_virtues(settlement: &mut Document, range: Range<i32>) {
    let i = r(range) as usize;
    let virtues: Vec<&&str> = VIRTUES16.choose_multiple(&mut rand::thread_rng(), i).collect();
    settlement.insert("virtues", virtues);
}


pub static ART_TOPICS: &'static [&'static str] = &[
    "bison",
    "eagles",
    "crows",
    "ravens",
    "deer",
    "rabbits",
    "spiders",
    "worms",
    "wolves",
    "lions",
    "elephants",
    "bears",
    "salmon",
    "dogs",
    "a castle",
    "a stone wall",
    "a battle",
    "a marketplace",
    "a library",
    "a tavern",
    "a windmill",
    "archers",
    "warriors",
    "knights",
    "bakers",
    "cooks",
    "horses",
    "swords",
];

pub fn gen_art_depictions(settlement: &mut Document, range: Range<i32>) {
    let vec1 = vec![
        format!("{}", random_item(LIVESTOCK16)),
        format!("{}", random_item(CROP32)),
        format!("{}", random_item(ART_TOPICS)),
        format!("{}", random_item(ART_TOPICS)),
        format!("{}", random_item(ART_TOPICS)),
    ];
    let affinity: Vec<&String> = vec1.choose_multiple(&mut rand::thread_rng(), r(range) as usize).collect();
    settlement.insert("affinity", &affinity);
}

pub fn gen_affinity(settlement: &mut Document, range: Range<i32>) {
    let vec1 = vec![
        format!("{}", random_item(METAL8)),
        format!("{}", random_item(LIVESTOCK16)),
        format!("{}", random_item(CROP32)),
        format!("{}", random_item(WOOD8)),
        format!("{}", random_item(STONE8)),
        format!("{}", random_item(CLOTH4)),
    ];
    let affinity: Vec<&String> = vec1.choose_multiple(&mut rand::thread_rng(), r(range) as usize).collect();
    settlement.insert("affinity", &affinity);
}

pub fn gen_dislikes(settlement: &mut Document, range: Range<i32>) {
    let vec1 = vec![
        format!("{}", random_item(METAL8)),
        format!("{}", random_item(LIVESTOCK16)),
        format!("{}", random_item(CROP32)),
        format!("{}", random_item(WOOD8)),
        format!("{}", random_item(STONE8)),
        format!("{}", random_item(CLOTH4)),
    ];
    let affinity: Vec<&String> = vec1.choose_multiple(&mut rand::thread_rng(), r(range) as usize).collect();
    settlement.insert("dislikes", &affinity);
}

pub fn gen_appearance(settlement: &mut Document, range: Range<i32>) {
    let vec1 = vec![
        format!("{} hair", random_item_v(vec![
            "carrot-red", "auburn",
            "golden", "sandy",
            "chestnut", "hazel",
            "jet-black", "pitch-black",
            "bone-white",
            "wild", "wispy", "sleek", "wiry",
        ])),
        format!("a {} face", random_item_v(vec![
            "round", "squarish", "pointed", "pinched",
        ])),
        format!("a {} posture", random_item_v(vec![
            "relaxed", "upright", "sulking", "graceful", "twisted",
        ])),
        format!("{} eyes", random_item_v(vec![
            "glinting", "sunken", "cheerful", "frosty", "intense", "wide", "dull",
        ])),
        format!("{}", random_item_v(vec![
            "bushy eyebrows", "thin eyebrows", "arched eyebrows",
            "a thin mouth", "a wide mouth",
            "a long nose", "a squat nose",
            "large ears",
            "dimpled cheeks",
        ])),
    ];
    let affinity: Vec<&String> = vec1.choose_multiple(&mut rand::thread_rng(), r(range) as usize).collect();
    settlement.insert("appearance", &affinity);
}

pub fn gen_art_name() -> String {
    let vec1 = vec![
        format!("The {x} of {n}", n=random_item(NAMES_FA), x=random_item_v(vec![
            "Lament", "Sorrow",
            "Rage", "Anger",
            "Despair", "Daring", "Fear",
            "Happiness", "Joy",

            "Instruction", "Admonishment", "Comfort",

            "Glory",
            "Path", "Journey",

            "Story", "Tale", "Life",

            "Veil", "Mystery", "Enigma", "Problem", "Riddle",
        ])),
    ];
    vec1.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn gen_random_industry() -> String {
    let vec1 = vec![
        format!("refining {}", random_item(METAL8)),
        format!("keeping {}", random_item(LIVESTOCK16)),
        format!("growing {}", random_item(CROP32)),
        format!("felling {} trees", random_item(WOOD8)),
        format!("quarrying {}", random_item(STONE8)),
        format!("producing {} cloth", random_item(CLOTH4)),
    ];
    return vec1.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn move_to(settlement: &Document, person: &mut Document) {
    person.insert("xy", settlement.get_array("xy").unwrap());
    person.insert("loc", settlement.get_array("loc").unwrap());
}


pub fn get_xy(doc: &Document) -> (i32, i32) {
    let q: &Array = doc.get_array("xy").unwrap();
    let xx = q.get(0).unwrap().as_i32().unwrap();
    let yy = q.get(1).unwrap().as_i32().unwrap();

    let xy = (xx, yy);
    (xx, yy)
}

pub fn move_to_contents(all: &Collection<Document>, mover: &Document, destination: &Document) {

    move_to_field(all, mover, destination, "contents", "contents");
}

pub fn move_to_field(all: &Collection<Document>, mover: &Document, destination: &Document, field_from: &str, field_to: &str) {
// old where
    let ref_old_where_result = mover.get_document("where");
    if ref_old_where_result.is_ok() {
        let ref_old_where = ref_old_where_result.unwrap();
        let old_where_id = ref_old_where.get_object_id("_id").unwrap();
        // remove self from old where
        all.update_one(
            doc! { "_id": old_where_id },
            doc! { "$pull": { field_from: refer(mover) } },
            None).unwrap();
    }

    // add self to new place
    all.update_one(
        doc! { "_id": destination.get_object_id("_id").unwrap()  },
        doc! { "$push": { field_to: &mover } },
        None).unwrap();
    set_fields(all, &mover, doc! {
        "xy": destination.get_array("xy").unwrap(),
        "loc": destination.get_array("loc").unwrap(),
        "where": refer(&destination),
    });
}
