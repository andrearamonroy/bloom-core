
use chrono::{Duration, Local, NaiveDate};

#[derive(Debug)]
struct Period {
    start: NaiveDate,
    end: NaiveDate,
}

impl Period {
    fn length(&self) -> i64 {
        (self.end - self.start).num_days() + 1
    }

}

fn date(year: i32, month: u32, day: u32)-> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap() 
}

fn days_until_next(cycle_length: i64, cycle_day: i64) -> i64 {
    //return lines should not have a semicolong at the end
    cycle_length - cycle_day
}

fn average_ cycle(periods: &Vec<Period>) -> i64 {
     let mut total = 0;
     
     for i in 1..periods.len(){
        let gap = (periods[i].start - periods[i - 1].start).num_days();
        total += gap;
     }
     total / (periods.len() as i64 - 1)
}

fn main() {

    //let p = Period{ start: date(2026,10,14), end: date(2026,10,18)};
    //println!("{:?}", p);
    //println!("Your period lasted {} days", p.length());

    let periods = vec! [
    Period {start : date(2026,7,21), end: date(2026,7,25 )},
    Period {start : date(2026,8,19), end: date(2026,8,23 )},
    Period {start : date(2026,9,17), end: date(2026,9, 21 )},
    ];

    //wihtout the & -- means give the list to the loop and its gone afterwards
    //witrh the &periods -- means lends the loop the list but you stil own it
    for p in &periods {
        println!(" {} -> {} ({} days)", p.start, p.end, p.length());
    }

    //naiveDate is a type from the chrono library that holds just the calendar date
    //year,month,day- no time or timezone that's why its called naive

    //form_ymd= make one FROM Year, Month, Day
    //opt = returns an optional
    //why an opt? because we could pass something line Feb 30 and instead of crashing it returns Option<NaiveDate>
    let last_start = NaiveDate::from_ymd_opt(2026,9,13).unwrap();

    //asks my laptop the current date and time something like “Oct 13, 2026, 12:59 PM, Mountain Time.”
    //.date_naive() drops the time and the time zone, leaving just the NaiveDate “2026-10-13.”
    let today = Local::now().date_naive();

    //chrono lets you subtract one date form another. the result is a duration(a span of time)
    //like 26 days worth of time
    //if your period started today that's day 0 but we wanted it to say 1 so + 1
    let cycle_day = (today - last_start).num_days() + 1; 
    let next_start = last_start + Duration::days(28);
    let days_left = (next_start - today).num_days();

    if days_left < 0 {
        println!("your period is {} days late",days_left.abs());
    } else if days_left <= 3{
        println!("pack some supplies! 🌸 Only {days_left} days to go ");
    } else {
        println!("next period in {days_left} days");
    }

    
/*

    println!("Today is cycle day {cycle_day}");
    println!("Next period {}", next_start.format("%A, %B %-d"));
    println!("That is in {days_left} days");

    let days_left = days_until_next(28, 11);
    println!("your next period is in {days_left} days");
*/
}
