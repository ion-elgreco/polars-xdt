use holidays::{Builder, Country};
use polars::prelude::*;
use polars::{
    error::{PolarsError, PolarsResult},
    series::Series,
};

pub fn to_holiday_range(start: i32, end: i32, countries: Vec<String>) -> PolarsResult<Series> {
    let mut country_vec: Vec<Country> = Vec::new();
    for country in countries {
        country_vec.push(
            country.parse().map_err(|err: holidays::Error| {
                PolarsError::ComputeError(err.to_string().into())
            })?,
        )
    }

    let holiday_map = Builder::new()
        .years(start..end)
        .countries(&country_vec)
        .build()
        .map_err(|err: holidays::Error| PolarsError::ComputeError(err.to_string().into()))?;

    let mut temp_vec = Vec::new();
    for country in country_vec {
        match holiday_map.get(&country) {
            Some(map) => {
                for (_, dates) in map {
                    for (date, _) in dates {
                        temp_vec.push(date.clone())
                    }
                }
            }
            None => continue,
        };
    }

    let ca = DateChunked::new("holidays", temp_vec);
    Ok(ca.into_series())
}
