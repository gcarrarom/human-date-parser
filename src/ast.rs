use chrono::Month;
use pest_consume::{match_nodes, Error, Parser as ConsumeParser};
use pest_derive::Parser;

use crate::{InternalError, ParseError};

type ParserResult<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

fn ordinal_from_str(s: &str) -> Option<u32> {
    let s = s.to_ascii_lowercase();
    if let Some(n) = parse_numeric_ordinal(&s) { return Some(n); }
    if let Some(n) = parse_word_ordinal(&s) { return Some(n); }
    None
}

fn parse_numeric_ordinal(s: &str) -> Option<u32> {
    if s.ends_with("st") || s.ends_with("nd") || s.ends_with("rd") || s.ends_with("th") {
        s[..s.len()-2].parse().ok()
    } else {
        None
    }
}

fn parse_tens_base(word: &str) -> Option<u32> {
    match word {
        "twenty" => Some(20),
        "thirty" => Some(30),
        "forty" => Some(40),
        "fifty" => Some(50),
        "sixty" => Some(60),
        "seventy" => Some(70),
        "eighty" => Some(80),
        "ninety" => Some(90),
        _ => None,
    }
}

fn parse_units_ordinal(word: &str) -> Option<u32> {
    match word {
        "first" => Some(1),
        "second" => Some(2),
        "third" => Some(3),
        "fourth" => Some(4),
        "fifth" => Some(5),
        "sixth" => Some(6),
        "seventh" => Some(7),
        "eighth" => Some(8),
        "ninth" => Some(9),
        _ => None,
    }
}

fn parse_word_ordinal(s: &str) -> Option<u32> {
    match s {
        // Handle 1-20 directly
        "first" => Some(1),
        "second" => Some(2),
        "third" => Some(3),
        "fourth" => Some(4),
        "fifth" => Some(5),
        "sixth" => Some(6),
        "seventh" => Some(7),
        "eighth" => Some(8),
        "ninth" => Some(9),
        "tenth" => Some(10),
        "eleventh" => Some(11),
        "twelfth" => Some(12),
        "thirteenth" => Some(13),
        "fourteenth" => Some(14),
        "fifteenth" => Some(15),
        "sixteenth" => Some(16),
        "seventeenth" => Some(17),
        "eighteenth" => Some(18),
        "nineteenth" => Some(19),
        "twentieth" => Some(20),
        // Handle compound patterns
        _ => parse_compound_ordinal(s),
    }
}

fn parse_tens_ordinal(word: &str) -> Option<u32> {
    match word {
        "twentieth" => Some(20),
        "thirtieth" => Some(30),
        "fortieth" => Some(40),
        "fiftieth" => Some(50),
        "sixtieth" => Some(60),
        "seventieth" => Some(70),
        "eightieth" => Some(80),
        "ninetieth" => Some(90),
        _ => None,
    }
}

fn parse_compound_ordinal(s: &str) -> Option<u32> {
    if let Some(hyphen_pos) = s.find('-') {
        let tens_part = &s[..hyphen_pos];
        let units_part = &s[hyphen_pos+1..];

        let tens_value = parse_tens_base(tens_part)?;
        let units_value = parse_units_ordinal(units_part)?;

        Some(tens_value + units_value)
    }
    else {
        parse_tens_ordinal(s)
    }
}

pub fn build_ast_from(str: &str) -> Result<HumanTime, ParseError> {
    let result = DateTimeParser::parse(Rule::HumanTime, &str)
        .and_then(|result| result.single())
        .map_err(|_| ParseError::InvalidFormat)?;

    DateTimeParser::HumanTime(result)
        .map_err(|_| ParseError::InternalError(InternalError::FailedToBuildAst))
}

#[derive(Parser)]
#[grammar = "date_time.pest"]
pub(crate) struct DateTimeParser;

#[pest_consume::parser]
impl DateTimeParser {
    pub(crate) fn HumanTime(input: Node) -> ParserResult<HumanTime> {
        Ok(match_nodes!(input.into_children();
            [DateTime(dt)] => HumanTime::DateTime(dt),
            [Date(d)] => HumanTime::Date(d),
            [Time(t)] => HumanTime::Time(t),
            [In(i)] => HumanTime::In(i),
            [Ago(a)] => HumanTime::Ago(a),
            [Now(_)] => HumanTime::Now,
        ))
    }

    fn DateTime(input: Node) -> ParserResult<DateTime> {
        Ok(match_nodes!(input.into_children();
            [Date(date), Time(time)] => DateTime{ date, time },
            [Time(time), Date(date)] => DateTime{ date, time },
        ))
    }

    fn IsoDate(input: Node) -> ParserResult<IsoDate> {
        Ok(match_nodes!(input.into_children();
            [Num(year), Num(month), Num(day)] => IsoDate{year, month, day},
        ))
    }

    fn Date(input: Node) -> ParserResult<Date> {
        Ok(match_nodes!(input.into_children();
            [Today(_)] => Date::Today,
            [Tomorrow(_)] => Date::Tomorrow,
            [Overmorrow(_)] => Date::Overmorrow,
            [Yesterday(_)] => Date::Yesterday,
            [IsoDate(iso)] => Date::IsoDate(iso),
            [Num(d), Month_Name(m), Num(y)] => Date::DayMonthYear(d, m, y),
            [Num(d), Month_Name(m)] => Date::DayMonth(d, m),
            [RelativeSpecifier(r), Week(_), Weekday(wd)] => Date::RelativeWeekWeekday(r, wd),
            [RelativeSpecifier(r), TimeUnit(tu)] => Date::RelativeTimeUnit(r, tu),
            [RelativeSpecifier(r), Weekday(wd)] => Date::RelativeWeekday(r, wd),
            [Weekday(wd)] => Date::UpcomingWeekday(wd),
            [OrdinalTimeUnitOf((ordinal, time_unit, datetime_ref))] => Date::OrdinalTimeUnitOf(ordinal, time_unit, datetime_ref),
        ))
    }

    fn Week(input: Node) -> ParserResult<Week> {
        Ok(Week {})
    }

    fn Ago(input: Node) -> ParserResult<Ago> {
        Ok(match_nodes!(input.into_children();
            [Duration(d)] => Ago::AgoFromNow(d),
            [Duration(d), HumanTime(ht)] => Ago::AgoFromTime(d, Box::new(ht)),
        ))
    }

    fn Now(input: Node) -> ParserResult<Now> {
        Ok(Now {})
    }

    fn Today(input: Node) -> ParserResult<Today> {
        Ok(Today {})
    }

    fn Tomorrow(input: Node) -> ParserResult<Tomorrow> {
        Ok(Tomorrow {})
    }

    fn Yesterday(input: Node) -> ParserResult<Yesterday> {
        Ok(Yesterday {})
    }

    fn Overmorrow(input: Node) -> ParserResult<Overmorrow> {
        Ok(Overmorrow {})
    }

    fn Time(input: Node) -> ParserResult<Time> {
        Ok(match_nodes!(input.into_children();
            [Num(h), Num(m)] => Time::HourMinute(h, m),
            [Num(h), Num(m), Num(s)] => Time::HourMinuteSecond(h, m, s),
        ))
    }

    fn In(input: Node) -> ParserResult<In> {
        Ok(match_nodes!(input.into_children();
            [Duration(d)] => In(d),
        ))
    }

    fn Duration(input: Node) -> ParserResult<Duration> {
        Ok(match_nodes!(input.into_children();
            [Quantifier(q)..] => Duration(q.collect()),
            [SingleUnit(su)] => Duration(vec![su]),
        ))
    }

    fn SingleUnit(input: Node) -> ParserResult<Quantifier> {
        Ok(match_nodes!(input.into_children();
            [TimeUnit(u)] => match u {
                TimeUnit::Year => Quantifier::Year(1),
                TimeUnit::Month => Quantifier::Month(1),
                TimeUnit::Week => Quantifier::Week(1),
                TimeUnit::Day => Quantifier::Day(1),
                TimeUnit::Hour => Quantifier::Hour(1),
                TimeUnit::Minute => Quantifier::Minute(1),
                TimeUnit::Second => Quantifier::Second(1),
            }
        ))
    }

    fn RelativeSpecifier(input: Node) -> ParserResult<RelativeSpecifier> {
        Ok(match_nodes!(input.into_children();
            [This(_)] => RelativeSpecifier::This,
            [Next(_)] => RelativeSpecifier::Next,
            [Last(_)] => RelativeSpecifier::Last,
        ))
    }

    fn This(input: Node) -> ParserResult<This> {
        Ok(This {})
    }

    fn Next(input: Node) -> ParserResult<Next> {
        Ok(Next {})
    }

    fn Last(input: Node) -> ParserResult<Last> {
        Ok(Last {})
    }

    fn Num(input: Node) -> ParserResult<u32> {
        input.as_str().parse::<u32>().map_err(|e| input.error(e))
    }

    fn Quantifier(input: Node) -> ParserResult<Quantifier> {
        Ok(match_nodes!(input.into_children();
            [Num(n), TimeUnit(u)] => match u {
                TimeUnit::Year => Quantifier::Year(n),
                TimeUnit::Month => Quantifier::Month(n),
                TimeUnit::Week => Quantifier::Week(n),
                TimeUnit::Day => Quantifier::Day(n),
                TimeUnit::Hour => Quantifier::Hour(n),
                TimeUnit::Minute => Quantifier::Minute(n),
                TimeUnit::Second => Quantifier::Second(n),
            }
        ))
    }

    fn TimeUnit(input: Node) -> ParserResult<TimeUnit> {
        if let Some(rule) = input.children().next() {
            Ok(match rule.as_rule() {
                Rule::Year => TimeUnit::Year,
                Rule::Month => TimeUnit::Month,
                Rule::Week => TimeUnit::Week,
                Rule::Day => TimeUnit::Day,
                Rule::Hour => TimeUnit::Hour,
                Rule::Minute => TimeUnit::Minute,
                Rule::Second => TimeUnit::Second,
                _ => unreachable!(),
            })
        } else {
            Err(input.error("Unreachable"))
        }
    }

    fn Weekday(input: Node) -> ParserResult<Weekday> {
        if let Some(rule) = input.children().next() {
            Ok(match rule.as_rule() {
                Rule::Monday => Weekday::Monday,
                Rule::Tuesday => Weekday::Tuesday,
                Rule::Wednesday => Weekday::Wednesday,
                Rule::Thursday => Weekday::Thursday,
                Rule::Friday => Weekday::Friday,
                Rule::Saturday => Weekday::Saturday,
                Rule::Sunday => Weekday::Sunday,
                _ => unreachable!(),
            })
        } else {
            Err(input.error("Unreachable"))
        }
    }

    fn Month_Name(input: Node) -> ParserResult<Month> {
        if let Some(rule) = input.children().next() {
            Ok(match rule.as_rule() {
                Rule::January => Month::January,
                Rule::February => Month::February,
                Rule::March => Month::March,
                Rule::April => Month::April,
                Rule::May => Month::May,
                Rule::June => Month::June,
                Rule::July => Month::July,
                Rule::August => Month::August,
                Rule::September => Month::September,
                Rule::October => Month::October,
                Rule::November => Month::November,
                Rule::December => Month::December,
                _ => unreachable!(),
            })
        } else {
            Err(input.error("Unreachable"))
        }
    }

    fn OrdinalTimeUnitOf(input: Node) -> ParserResult<(Ordinal, TimeUnit, DateTimeReference)> {
        Ok(match_nodes!(input.into_children();
            [Ordinal(ordinal), TimeUnit(time_unit), DateTimeReference(datetime_ref)] => (ordinal, time_unit, datetime_ref),
        ))
    }

    fn DateTimeReference(input: Node) -> ParserResult<DateTimeReference> {
        Ok(match_nodes!(input.into_children();
            [MonthSpec(month_spec)] => DateTimeReference::MonthYear(month_spec, None),
            [MonthSpec(month_spec), YearSpec(year_spec)] => DateTimeReference::MonthYear(month_spec, Some(year_spec)),
            [Duration(duration)] => DateTimeReference::Ago(duration),
            [RelativeSpecifier(relative), TimeUnit(time_unit)] => DateTimeReference::RelativeTimeUnit(relative, time_unit),
            [TimeUnit(time_unit)] => DateTimeReference::TheTimeUnit(time_unit),
            [Today(_)] => DateTimeReference::Today,
            [Tomorrow(_)] => DateTimeReference::Tomorrow,
            [Yesterday(_)] => DateTimeReference::Yesterday,
            [Overmorrow(_)] => DateTimeReference::Overmorrow,
            [Now(_)] => DateTimeReference::Now,
        ))
    }

    fn Ordinal(input: Node) -> ParserResult<Ordinal> {
        let text = input.as_str();
        match text.to_ascii_lowercase().as_str() {
            "last" => Ok(Ordinal::Last),
            _ => ordinal_from_str(text)
                .map(|n| if n == 1 { Ordinal::First } else { Ordinal::Nth(n) })
                .ok_or_else(|| input.error("Invalid ordinal"))
        }
    }



    fn MonthSpec(input: Node) -> ParserResult<MonthSpec> {
        let text = input.as_str();
        if text == "month" || text == "the month" {
            Ok(MonthSpec::Current)
        } else {
            Ok(match_nodes!(input.into_children();
                [Month_Name(month)] => MonthSpec::Absolute(month),
                [RelativeSpecifier(relative), Month_Name(month)] => MonthSpec::Relative(relative, month),
                [RelativeSpecifier(relative)] => MonthSpec::RelativeCurrent(relative),
            ))
        }
    }

    fn YearSpec(input: Node) -> ParserResult<YearSpec> {
        Ok(match_nodes!(input.into_children();
            [RelativeSpecifier(relative)] => YearSpec::Relative(relative),
            [Num(year)] => YearSpec::Absolute(year),
        ))
    }
}

#[derive(Debug)]
pub enum HumanTime {
    DateTime(DateTime),
    Date(Date),
    Time(Time),
    In(In),
    Ago(Ago),
    Now,
}

#[derive(Debug)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[derive(Debug)]
pub struct IsoDate {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug)]
pub enum Date {
    Today,
    Tomorrow,
    Overmorrow,
    Yesterday,
    IsoDate(IsoDate),
    DayMonthYear(u32, Month, u32),
    DayMonth(u32, Month),
    RelativeWeekWeekday(RelativeSpecifier, Weekday),
    RelativeTimeUnit(RelativeSpecifier, TimeUnit),
    RelativeWeekday(RelativeSpecifier, Weekday),
    UpcomingWeekday(Weekday),
    OrdinalTimeUnitOf(Ordinal, TimeUnit, DateTimeReference),
}

#[derive(Debug)]
struct Today;
#[derive(Debug)]
struct Tomorrow;
#[derive(Debug)]
struct Yesterday;
#[derive(Debug)]
struct Overmorrow;

#[derive(Debug)]
pub enum Time {
    HourMinute(u32, u32),
    HourMinuteSecond(u32, u32, u32),
}

#[derive(Debug)]
pub struct In(pub Duration);

#[derive(Debug)]
pub enum Ago {
    AgoFromNow(Duration),
    AgoFromTime(Duration, Box<HumanTime>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Duration(pub Vec<Quantifier>);

#[derive(Debug)]
struct Now;

#[derive(Debug, Clone, Copy)]
pub enum RelativeSpecifier {
    This,
    Next,
    Last,
}

#[derive(Debug)]
struct This;
#[derive(Debug)]
struct Next;
#[derive(Debug)]
struct Last;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Quantifier {
    Year(u32),
    Month(u32),
    Week(u32),
    Day(u32),
    Hour(u32),
    Minute(u32),
    Second(u32),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<Weekday> for chrono::Weekday {
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Monday => chrono::Weekday::Mon,
            Weekday::Tuesday => chrono::Weekday::Tue,
            Weekday::Wednesday => chrono::Weekday::Wed,
            Weekday::Thursday => chrono::Weekday::Thu,
            Weekday::Friday => chrono::Weekday::Fri,
            Weekday::Saturday => chrono::Weekday::Sat,
            Weekday::Sunday => chrono::Weekday::Sun,
        }
    }
}

#[derive(Debug)]
struct Week {}

#[derive(Debug)]
pub enum Ordinal {
    First,
    Last,
    Nth(u32),
}

#[derive(Debug)]
pub enum MonthSpec {
    Absolute(Month),
    Relative(RelativeSpecifier, Month),
    RelativeCurrent(RelativeSpecifier),
    Current,
}

#[derive(Debug)]
pub enum YearSpec {
    Relative(RelativeSpecifier),
    Absolute(u32),
}

#[derive(Debug)]
pub enum DateTimeReference {
    MonthYear(MonthSpec, Option<YearSpec>),
    Ago(Duration),
    RelativeTimeUnit(RelativeSpecifier, TimeUnit),
    TheTimeUnit(TimeUnit),
    Today,
    Tomorrow,
    Yesterday,
    Overmorrow,
    Now,
}
