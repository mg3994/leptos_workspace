use std::ops::RangeInclusive;
use rig::vector_store::request::SearchFilter;
use serde_json;

#[derive(Clone, Default)]
pub struct PgSearchFilter {
    condition: String,
    values: Vec<serde_json::Value>,
}

impl SearchFilter for PgSearchFilter {
    type Value = serde_json::Value;

    fn eq(key: String, value: Self::Value) -> Self {
        Self {
            condition: format!("{key} = $"),
            values: vec![value],
        }
    }

    fn gt(key: String, value: Self::Value) -> Self {
        Self {
            condition: format!("{key} > $"),
            values: vec![value],
        }
    }

    fn lt(key: String, value: Self::Value) -> Self {
        Self {
            condition: format!("{key} < $"),
            values: vec![value],
        }
    }

    fn and(self, rhs: Self) -> Self {
        Self {
            condition: format!("({}) AND ({})", self.condition, rhs.condition),
            values: self.values.into_iter().chain(rhs.values).collect(),
        }
    }

    fn or(self, rhs: Self) -> Self {
        Self {
            condition: format!("({}) OR ({})", self.condition, rhs.condition),
            values: self.values.into_iter().chain(rhs.values).collect(),
        }
    }
}

impl PgSearchFilter {
    pub fn into_clause(self) -> (String, Vec<serde_json::Value>) {
        (self.condition, self.values)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self {
            condition: format!("NOT ({})", self.condition),
            values: self.values,
        }
    }

    pub fn gte(key: String, value: <Self as SearchFilter>::Value) -> Self {
        Self {
            condition: format!("{key} >= ?"),
            values: vec![value],
        }
    }

    pub fn lte(key: String, value: <Self as SearchFilter>::Value) -> Self {
        Self {
            condition: format!("{key} <= ?"),
            values: vec![value],
        }
    }

    pub fn is_null(key: String) -> Self {
        Self {
            condition: format!("{key} is null"),
            ..Default::default()
        }
    }

    pub fn is_not_null(key: String) -> Self {
        Self {
            condition: format!("{key} is not null"),
            ..Default::default()
        }
    }

    pub fn between<T>(key: String, range: RangeInclusive<T>) -> Self
    where
        T: std::fmt::Display + Into<serde_json::Number> + Copy,
    {
        let lo = range.start();
        let hi = range.end();
        Self {
            condition: format!("{key} between {lo} and {hi}"),
            ..Default::default()
        }
    }

    pub fn member(key: String, values: Vec<<Self as SearchFilter>::Value>) -> Self {
        let placeholders = values
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(",");
        Self {
            condition: format!("{key} is in ({placeholders})"),
            values,
        }
    }

    // String matching ops
    /// Tests whether the value at `key` matches the (case-sensitive) pattern
    /// `pattern` should be a valid SQL string pattern, with '%' and '_' as wildcards
    pub fn like(key: String, pattern: &'static str) -> Self {
        Self {
            condition: format!("{key} like {pattern}"),
            ..Default::default()
        }
    }

    /// Tests whether the value at `key` matches the SQL regex pattern
    /// `pattern` should be a valid regex
    pub fn similar_to(key: String, pattern: &'static str) -> Self {
        Self {
            condition: format!("{key} similar to {pattern}"),
            ..Default::default()
        }
    }
}