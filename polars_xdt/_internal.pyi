from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import polars as pl

__version__: str

def holiday_range(start: int, end: int, countries: list[str]) -> pl.Series: ...
