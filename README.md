# sad
Console application which reads project's data, transforms the data
according to the following instructions and finally outputs the results to console.

The application supports the following command line arguments (only the first
one is mandatory):
```
-File <path>             full path to the input file
-SortByStartDate         sort results by column "Start date" in ascending order
-Project <project id>    filter results by column "Project"
```
The following requirements define the program functionality
and refer to the data sample below:

* Input data is tab-separated UTF-8 text with a header row.

* It only includes the columns listed below.

* Dates (Start date) and money (Savings amount) values conform to certain
format presented below.

* Columns "Savings amount" and "Currency" can have missing values denoted
as NULL. Those printed as empty strings.

* Column "Complexity" has a certain set of values (Simple, Moderate, Hazardous).

* The output should also have a header line.

* Lines that are empty or start with comment mark # are skipped.

* Order (but not names) of columns might be changed in future.

* In case of an invalid source value (in a date, money or Complexity column) a
descriptive error message should be printed to console and the program terminated.

```
/***********************************************************************************************************************************************/
# Input file contents are between the two lines marked with stars
Project	Description	Start date	Category	Responsible	Savings amount	Currency	Complexity
2	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks	NULL	NULL	Simple
3	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR	Moderate
3	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR	Moderate
4	Decrease production related non-categorized side costs	2013-01-01 00:00:00.000	Dairy	Daisy Milks	11689.322459	EUR	Hazardous
4	Decrease production related non-categorized side costs	2013-01-01 00:00:00.000	Dairy	Daisy Milks	11689.322459	EUR	Hazardous
5	Stop using Kryptonite in production	2013-04-01 00:00:00.000	Dairy	Clark Kent	NULL	NULL	Moderate
6	Black and white logo paper	2012-06-01 00:00:00.000	Office supplies	Clark Kent	4880.199567	EUR	Simple
6	Black and white logo paper	2012-06-01 00:00:00.000	Office supplies	Clark Kent	4880.199567	EUR	Simple
/***********************************************************************************************************************************************/
```

## Quick start
Rust and Cargo should be installed.
Run tests
```
cargo test
```

Run binary
```
cargo run -- -File ExampleData.tsv -SortByStartDate -Project 3

```
Help
```
cargo run -- -help
```
