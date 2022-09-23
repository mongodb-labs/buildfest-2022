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

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use bson::{Array, Bson, Document};
use bson::oid::ObjectId;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use rand::Rng;
use rand::seq::SliceRandom;
use string_template::Template;

use crate::common::{*, add_fields, rand};
use crate::{common, SERVER};
use crate::generate::{*};
use crate::generate;

pub fn start_server() {
    println!("Listening: http://{}", SERVER);

    rouille::start_server(SERVER, move |request| {
        router!(request,
            // TODO no longer works, background thread
            // (GET) (/reset) => {
            //     println!("Hello world!");
            //     let everything = common::connect_to_everything().unwrap();
            //     generate::generate_new_world(&everything).unwrap();
            //     rouille::Response::redirect_302("/")
            // },
            (GET) (/) => {
                println!("Random document!");
                let id = get_random_id();
                if id.is_none() {
                    return rouille::Response::empty_404();
                } else {
                    return rouille::Response::redirect_302(format!("/{}", id.unwrap()));
                }
            },
            (GET) (/{id: String}) => {
                if (id.contains("favicon.ico")) {
                    return rouille::Response::empty_404();
                }
                println!("Document: {}", id);
                let html = record_to_html(&id);
                if html.is_none() {
                    return rouille::Response::redirect_302("/")
                } else {
                    return rouille::Response::html(html.unwrap());
                }
            },
            _ => rouille::Response::redirect_302("/")
        )
    });
}

pub fn get_random_id() -> Option<String> {
    let all = common::connect_to_everything().unwrap();

    // even distribution
    let target = random_item(&[
        "land",
        "settlement",
        "person",
        "artwork",
    ]);

    let mut cursor = all.aggregate(vec![
        doc! { "$match": { "type": {"$eq": target} } },
        doc! { "$sample": { "size": 1 } },
    ], None).unwrap();
    let next = cursor.next();
    if next.is_none() {
        return None;
    }
    let doc = next.expect("NONE").unwrap();
    let id = doc.get_object_id("_id").unwrap().to_string();
    Some(id)
}

pub fn record_to_html(id: &String) -> Option<String> {
    let oid = ObjectId::parse_str(&id).unwrap();
    let all = connect_to_everything().unwrap();

    let mut results2 = all.aggregate(vec![
        doc! { "$match": { "_id": oid } }
    ], None).unwrap();

    let next = results2.next();
    if next.is_none() {
        return None;
    }

    let doc = next.expect("NONE").unwrap();
    let desc = record_to_div(doc);

    let css = r#"
          body {
            font-family: 'IM Fell English', serif;
            font-size: 140%;
            background: #b2b2b2;
            background: #a8a8a8;
            background: #d1d1d1;
          }
          div.page {
            margin: 0.5in auto;
            width: 5in;
            min-height: 7in;
            padding: 0.5in;
            box-shadow: 1px 1px 10px rgb(0 0 0 / 16%);
            border-radius: 3px;

            background: #eee1cd;
            background: rgb(233,221,187);

            background: linear-gradient(180deg, rgb(230 225 208) 0%, rgb(227 213 186) 100%);
            background: radial-gradient(rgb(237 233 217) 50%, rgb(254 244 223) 100%);
            background: radial-gradient(#fff7ee 50%, rgb(254 244 223) 100%);
          }
          div.page:first-letter {text-transform: uppercase}
          a:link, a:hover, a:visited {
            color: #13aa52;
            color: #12924f;
            color: #116149;
          }
          a {
            transition: color 275ms ease;

            text-decoration: none;
            font-weight: bold;
            transition: color 275ms ease;
            white-space:nowrap;
          }
          a:hover {
            text-decoration: underline;
            color: #13aa52;
          }

          a.reset {
            float: left;
            color: #f4b65d;
            display: block;
            border: 3px double #f4b65d;
            width: 26px;
            height: 26px;
            text-align: center;
            border-radius: 1in;
            border-style: double;
            margin: 5px;
            color: #e1e1e1;
            border: 3px double #e1e1e1;
            border: 2px solid #e1e1e1;
          }
          .debug {
            margin-top: 3in;
            color:#aaa;
            word-wrap: break-word;
            white-space: pre-wrap;
          }
          .map {
            position: relative;
            margin: 50px auto 0;

            padding: 20px;
            overflow: hidden;

            outline: 2px solid #f4b65d;
            outline: 2px solid black;
            xfont-size: 90%;
          }
          .map a {
            color: #f4b65d;
            color: black;
            color: inherit;

            width: 10px;
            height: 10px;
            display: block;
          }
          .map a:hover { text-decoration: none; }
          .icon {
            position: relative;
            width: 0px;
            height: 0px;
            text-align: center;
          }
          .disabled_shadow {
            box-shadow: inset 0 0 3px 8px rgb(237 233 217);
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: 10;
            pointer-events: none;
          }

          .forest {
            color: black;
          }
          .field {
            color: black;
          }
          .hills {
            color: black;
          }
          .water {
            color: black;
          }
          .settlement {
            color: red;
          }

    "#;

    let js = r#"

        var fn = function(){ window.location.href = '/'; };
        var time = 9000;
        var timeout = setTimeout(fn, time)
        document.onmousemove = function(){
          clearTimeout(timeout);
          timeout = setTimeout(fn, time);
        }
        document.onscroll = function(){
          clearTimeout(timeout);
          timeout = setTimeout(fn, time);
        }
    "#;

    let js = ""; // disable refresh

    let html = format!(r#"
    <html>
    <head>
        <meta charset="utf-8">
        <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=IM+Fell+English">
        <style>{css}
        </style>
        <script>{js}
        </script>
    </head>
    <body>

    <a href="/" class="reset">↩</a>
    {body}
    </body>
    </html>
    "#, body=desc, css=css);
    // <a href="/reset" class="reset">#</a>
    Some(html)
}


pub fn record_to_div(doc: Document) -> String {
    let strings = doc!{
        "settlement": "{name} was established in the year {founded}, \
            and is home to {population} people. \
            Its citizens are primarily occupied with {industry}. \
            They have an affinity for {affinity}. \
            They admire {virtues}. \
            <p> Here may be found {contents}.",

        "land": "{name}. \
            <p> Here may be found {contents}.
            <p> Deep in the land rests {grave}.",

        "person": "{name} is a {profession} with \
            {appearance}, \
            a liking of {affinity}, \
            and a commitment to {virtues}. \
            {death}\
            <p> {name} possesses {contents}.",

        "artwork": "{name} is a {medium} created by {author}. \
            It extols {virtues} \
            and depicts {affinity}. \
            It conveys {feeling}.",
    };
    let event_strings = doc!{
        "founded": "{settlement} was established on {land}",
        "arose": "{person} arose in {birthplace}",
        "famine": "{cause} {settlement}, killing {deaths}",
        "life": "{person} {desc}",
        "moved": "{person} moved from {from} to {to} {why}",
        "explore": "{person} discovered {to}, travelling from {from}",
        "died": "{person} died in {where}",
        "created": "{person} {verb} {item}",
        "viewed": "{person} viewed {artwork} and {effect}",
        "feast": "{settlement} {desc}",
    };

    let doc_type = doc.get_str("type").unwrap();
    let template = strings.get_str(doc_type).unwrap().to_string();
    let desc = populate_template(&doc, template);

    let desc = desc.replace("{contents}", "much to observe");



    let mut edesc= "".to_string();
    if doc.contains_key("events") {
        let events = doc.get_array("events").unwrap();

        for q in events {
            let event = q.as_document().unwrap();
            //let event_desc = event.get_str("text").unwrap().to_string();
            let event_template_id = event.get_str("template_id").unwrap();
            let event_importance_result = event.get_i32("importance");
            let mut event_importance = 10;
            if event_importance_result.is_ok() {
                event_importance = event_importance_result.unwrap();
            }
            if (event_importance < 2 && doc_type == "settlement") {
                continue;
            }
            let event_string = event_strings.get_str(event_template_id).unwrap().to_string();
            let event_string = format!("In {{year}}, {}.", event_string);
            let event_desc = populate_template(&event, event_string);
            edesc = format!("{}{}<br>", edesc, event_desc);
        }
    }

    let tilde = "<p style='text-align:center'>~</p>";

    let map = map_html(&doc, tilde);

    let desc = format!("\
    <div class='page'>
    {desc}
    {tilde}
    {edesc}
    {map}
    </div>
    <pre class='debug'>{doc:?}</pre>
    ");
    /*
     */
    desc
}

pub fn populate_template(doc: &Document, desc: String) -> String {
    let mut desc = desc;
    for k in doc.keys() {
        let key = format!("{{{}}}", k);
        if desc.contains(&key) {
            //println!("KEY:{}", k);
            let bson = doc.get(k);
            let bson = bson.unwrap();
            let replacement = to_replacement(bson);
            desc = desc.replace(&key, replacement.as_str());
        }
    }
    desc = desc.replace(" a a", " an a");
    desc = desc.replace(" a e", " an e");
    desc = desc.replace(" a i", " an i");
    desc = desc.replace(" a o", " an o");
    desc = desc.replace(" a u", " an u");
    desc
}

fn to_replacement(bson: &Bson) -> String {
    let mut replacement = "".to_string();
    let o = bson.as_str();
    if o.is_some() {
        replacement = o.unwrap().to_string();
    }
    let o = bson.as_i32();
    if o.is_some() {
        replacement = format!("{}", o.unwrap());
    }
    let o = bson.as_array();
    if o.is_some() {
        let x = o.unwrap();
        let mut count = 0;
        let len = x.len();
        for xx in x.into_iter() {
            count = count + 1;
            let v = to_replacement(xx); //xx "AA";
            if count == 1 {
                replacement = format!("{}", v).to_string();
            } else if count == len && len == 2 {
                replacement = format!("{} and {}", replacement, v).to_string();
            } else if count == len {
                replacement = format!("{}, and {}", replacement, v).to_string();
            } else {
                replacement = format!("{}, {}", replacement, v).to_string();
            }
        }
        if len == 0 {
            replacement = "nothing of note".to_string();
        }
    }
    let o = bson.as_document();
    if o.is_some() {
        let o = o.unwrap();
        replacement = to_link("name", o);
    }
    replacement
}

fn to_link(display_key: &str, o: &Document) -> String{
    let id = o.get_object_id("_id").unwrap().to_string();
    let name = o.get_str(display_key).unwrap().to_string();
    format!("<a href='/{id}'>{name}</a>")
}


fn map_html(doc: &Document, tilde: &str) -> String {
    let doc_type = doc.get_str("type").unwrap();
    let mut map = format!("");
    if doc_type == "settlement" || doc_type == "land" {
        let all = connect_to_everything().unwrap();

        let mut map_string = "".to_string();

        let (hx, hy) = get_xy(&doc);

        let num_across = 31;
        let spread = 14;

        let num_across = 21;
        let spacing = 20;
        let half_char = spacing /2;

        let amount = num_across * num_across;

        let results = all.aggregate(vec![
            doc! { "$geoNear": {
                "near": doc.get_array("loc").unwrap(),// common::loc(0,0),
                "distanceField": "dist",
            }},
            //unset_ignored,
            doc! { "$match": { "type": "land"}},
            doc! { "$limit": amount * 2 },
        ], None).unwrap();


        let width = spacing * num_across;
        let half_width = width / 2;
        for next in results {
            let doc = next.expect("NONE");
            let settlement = doc.get_str("status").unwrap() == "settled";

            let (xx, yy) = get_xy(&doc);
            let q: &Array = doc.get_array("map_offset").unwrap();
            let mut oxx = q.get(0).unwrap().as_f64().unwrap();// / 3.0;
            let mut oyy = q.get(1).unwrap().as_f64().unwrap();// / 3.0;
            if settlement {
                oxx = 0.0;
                oyy = -0.24;
            }
            // let oxx = 0.0;
            // let oyy = 0.0;
            // println!("{oxx} X {oyy}");

            let top = (xx - hx) * spacing + (spacing as f64 * oxx) as i32 + half_width - half_char;
            let left = (yy - hy) * spacing + (spacing as f64 * oyy) as i32 + half_width - half_char;
            let mut land_type = doc.get_str("land_type").unwrap();
            let mut land_symbol = to_link("symbol", &doc);


            if settlement {
                land_type = "settlement";
            }

            map_string = format!(
                "{map_string}<div class='icon {land_type}' style='top:{top}px; left:{left}px; width:{spread}px;'>{land_symbol}</div>");
        }
        //let size = width + 30
        // {tilde}
        map = format!("<div class='map' style='width:{width}px; height:{width}px;'><div class='shadow'></div>{}</div>", map_string);
    }
    map
}