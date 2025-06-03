use chrono::{self, Duration, NaiveDateTime, Timelike};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Act {
    Begin,
    Sleep,
    Wake,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Event {
    guard_id: Option<u32>,
    act: Option<Act>,
    start: Option<NaiveDateTime>,
    end: Option<NaiveDateTime>,
    minutes: u32,
}

impl Event {
    #[allow(dead_code)]
    fn new(
        guard_id: Option<u32>,
        act: Option<Act>,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
        minutes: u32,
    ) -> Self {
        Self {
            guard_id,
            act,
            start,
            end,
            minutes,
        }
    }
}

#[allow(dead_code)]
fn input() -> Vec<Event> {
    let mut eventlog: Vec<Event> = include_str!("../inputs/day04.txt")
        .lines()
        .map(|line| {
            // guard_id
            let mut guard_id: Option<u32> = None;
            if let (Some(i), Some(j)) = (line.find("Guard #"), line.find(" begins shift")) {
                guard_id = Some(line[i + 7..j].parse::<u32>().expect(
                    format!("cannot convert {} to u32", line[i + 7..j].to_string()).as_str(),
                ));
            }

            // act
            let act = if line.contains(&"begin") {
                Some(Act::Begin)
            } else if line.contains(&"sleep") {
                Some(Act::Sleep)
            } else if line.contains(&"wake") {
                Some(Act::Wake)
            } else {
                None
            };

            // start
            let start = Some(
                NaiveDateTime::parse_from_str(
                    &line[line.find("[").unwrap() + 1..line.find("]").unwrap()],
                    "%Y-%m-%d %H:%M",
                )
                .expect("cannot convert to NaiveDateTime"),
            );

            // end
            let end = None;

            // minutes
            let minutes = 0;

            Event::new(guard_id, act, start, end, minutes)
        })
        .collect();

    // sort by when
    eventlog.sort_by(|x, y| x.start.cmp(&y.start));

    // further adjusting
    let mut cached_guard_id: Option<u32> = None;
    for i in 0..eventlog.len() {
        // guard_id
        if let Some(guard_id) = eventlog[i].guard_id {
            cached_guard_id = Some(guard_id);
        } else {
            eventlog[i].guard_id = cached_guard_id;
        }

        if i > 0 {
            // end
            eventlog[i - 1].end = Some(eventlog[i].start.unwrap());

            // minutes
            eventlog[i - 1].minutes = (eventlog[i - 1].end.unwrap()
                - eventlog[i - 1].start.unwrap())
            .num_minutes() as u32;
        }
    }
    eventlog
}

#[allow(dead_code)]
pub fn part_one() {
    let eventlog = input();
    // count guard sleep minutes
    let sleep_minutes: HashMap<u32, u32> =
        eventlog
            .iter()
            .fold(HashMap::new(), |mut sleep_log: HashMap<u32, u32>, event| {
                if *event.act.as_ref().unwrap() == Act::Sleep {
                    *sleep_log.entry(event.guard_id.unwrap()).or_default() += event.minutes
                }
                sleep_log
            });
    let sleepiest_guard = *sleep_minutes.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;

    let minute_map: HashMap<u32, u32> = eventlog
        .iter()
        .filter(|event| event.guard_id == Some(sleepiest_guard) && event.act == Some(Act::Sleep))
        .fold(
            HashMap::new(),
            |mut minute_map: HashMap<u32, u32>, event| {
                for i in 0..event.minutes {
                    let current_minute =
                        (event.start.unwrap().time() + Duration::minutes(i as i64)).minute();
                    *minute_map.entry(current_minute).or_default() += 1;
                }
                minute_map
            },
        );
    let sleepiest_minute = *minute_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
    println!("{}", sleepiest_guard * sleepiest_minute);
}

#[allow(dead_code)]
pub fn part_two() {
    let eventlog = input();
    let guard_minute_map: HashMap<(u32, u32), u32> = eventlog
        .iter()
        .filter(|event| event.act == Some(Act::Sleep))
        .fold(
            HashMap::new(),
            |mut guard_minute_map: HashMap<(u32, u32), u32>, event: &Event| {
                for i in 0..event.minutes {
                    let current_minute =
                        (event.start.unwrap().time() + Duration::minutes(i as i64)).minute();
                    *guard_minute_map
                        .entry((event.guard_id.unwrap(), current_minute))
                        .or_default() += 1;
                }
                guard_minute_map
            },
        );

    let (sleepiest_guard, sleepiest_minute) = *guard_minute_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .0;
    println!("{}", sleepiest_guard * sleepiest_minute);
}
