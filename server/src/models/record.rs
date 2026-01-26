use crate::db::get_uuid;
use chrono::Utc;
use fastnum::decimal::UD64;
use uuid::Uuid;

pub struct Record {
    pub id: Uuid,
    pub stock: Uuid,
    pub date: chrono::DateTime<Utc>,
    pub open: UD64,
    pub close: UD64,
}

pub struct Month {
    pub id: Uuid,
    pub stock: Uuid,
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub open: UD64,
    pub close: UD64,
}

pub struct Week {
    pub id: Uuid,
    pub stock: Uuid,
    pub month: Uuid,
    pub week_num: i32,
    pub date: chrono::DateTime<Utc>,
    pub peak: UD64,
    pub days_up: i32,
    pub days_down: i32,
    pub open: UD64,
    pub close: UD64,
}

pub struct Report {
    pub player: Uuid,
    pub stock: Uuid,
    pub week: Uuid,
    pub open: UD64,
    pub close: UD64,
    pub cache: String,
}

impl Record {
    pub fn new(stock: Uuid, open: UD64, close: UD64) -> Self {
        Self {
            id: get_uuid(),
            stock,
            date: Utc::now(),
            open,
            close,
        }
    }
}
impl Month {
    pub fn new(stock: Uuid, start: chrono::DateTime<Utc>, open: UD64, close: UD64) -> Self {
        Self {
            id: get_uuid(),
            stock,
            start,
            open,
            close,
            end: Utc::now(),
        }
    }
}
impl Week {
    pub fn new(
        stock: Uuid,
        month: Uuid,
        week: i32,
        peak: UD64,
        up: i32,
        down: i32,
        open: UD64,
        close: UD64,
    ) -> Self {
        Self {
            id: get_uuid(),
            stock,
            month,
            week_num: week,
            date: Utc::now(),
            peak,
            days_up: up,
            days_down: down,
            open,
            close,
        }
    }
}
impl Report {
    pub fn new(player: Uuid, stock: Uuid, week: Uuid, open: UD64, close: UD64) -> Self {
        Self {
            player,
            stock,
            week,
            open,
            close,
            cache: "".to_string(),
        }
    }
}
