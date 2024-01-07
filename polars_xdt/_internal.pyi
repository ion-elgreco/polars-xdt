import polars as pl

__version__: str

def holiday_range(start: int, end: int, countries: list[str]) -> pl.Series: ...
