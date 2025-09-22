use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASECOND: i64 = 10_i64.pow(9);

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGASECOND)
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};

    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

fn main() {
    let start = datetime(1977, 6, 13, 0, 0, 0);
    let actual = after(start);
    let expected = datetime(2009, 2, 19, 1, 46, 40);

    dbg!(assert_eq!(actual, expected));
}
