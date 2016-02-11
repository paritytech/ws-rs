/// WebSocket client used for testing against the Autobahn Test Suite

extern crate ws;

use std::rc::Rc;
use std::cell::Cell;
use ws::{connect, CloseCode, Message, Result};

const AGENT: &'static str = "WS-RS";

fn main () {

    let total = get_case_count().unwrap();
    let mut case_id = 1;


    while case_id <= total {

        let case_url = format!("ws://127.0.0.1:9001/runCase?case={}&agent={}", case_id, AGENT);

        connect(case_url, |out| {
            move |msg| {
                out.send(msg)
            }
        }).unwrap();

        case_id += 1
    }

    update_reports().unwrap();
}

fn get_case_count() -> Result<u32> {

    // sadly we need to use a Cell because we need to set the total, and RC is immutable
    let total = Rc::new(Cell::new(0));

    try!(connect("ws://127.0.0.1:9001/getCaseCount", |out| {

        let my_total = total.clone();

        move |msg: Message| {

            let count = try!(msg.as_text());

            my_total.set(count.parse::<u32>().unwrap());

            out.close(CloseCode::Normal)
        }

    }));

    Ok(total.get())
}

fn update_reports() -> Result<()> {
    let report_url = format!("ws://127.0.0.1:9001/updateReports?agent={}", AGENT);

    connect(report_url, |out| {
        move |_| {
            out.close(CloseCode::Normal)
        }
    })
}
