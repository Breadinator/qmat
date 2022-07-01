# Matrix creation
The difference between `Matrix::new` and `matrix!(rows, cols, flat_data)` is basically nothing, both around 510 [picoseconds](https://en.wikipedia.org/wiki/Picosecond). This is expected because the latter is meant to be a convenient macro that expresses the exact same as the former.

Times for `matrix!([row0, row1, etc])` take over twice as long. This is expected because it expands to a `Matrix::from_rows` call which iterates over `[[T; N]; M]` to flatten it into `[T; N*M]`, then passes that into `Matrix::new`.

| Method | Mean | Graph |
|--------|-----------|-------|
|`Matrix::new`|513.17ps|![https://i.imgur.com/gsrARX6.png](https://i.imgur.com/gsrARX6.png)|
|`matrix!(rows, cols, flat_data)`|504.63ps|![https://i.imgur.com/uipZE5s.png](https://i.imgur.com/uipZE5s.png)|
|`matrix!([row0, row1, etc])`|1322.1ps|![https://i.imgur.com/6lF1nUS.png](https://i.imgur.com/6lF1nUS.png)|
