extern crate itertools;
use std::cmp;
use std::fmt;
use std::iter;
use std::vec;

use self::itertools::join;

macro_rules! own {
    ($($s:expr),*) => { ($($s.to_owned()), *) }
}

type Row = Vec<String>;

pub struct TableConfig {
    width: usize,
    padding: usize,
    border: (String, String, String),
}

impl Default for TableConfig {
    fn default() -> TableConfig {
        TableConfig {
            width: 80,
            padding: 1,
            border: own!("|", "-", "+"),
        }
    }
}

#[derive(Default)]
pub struct Table {
    title: Option<Row>,
    rows: Vec<Row>,
    config: TableConfig,
}

impl Table {
    pub fn new(config: TableConfig) -> Table {
        Table {
            title: None,
            rows: vec![],
            config: config,
        }
    }

    pub fn with_width(width: usize) -> Table {
        let mut config = TableConfig::default();
        config.width = width;
        Table::new(config)
    }

    pub fn set_title(&mut self, title: Row) {
        self.title = Some(title);
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
    pub fn add_rows(&mut self, rows: &mut Vec<Row>) {
        self.rows.append(rows);
    }

    fn dimensions(&self) -> Vec<usize> {
        let dimensions = self.title
            .iter()
            .chain(self.rows.iter())
            .map(|x| x.iter().map(|s| s.len()).collect::<Vec<_>>())
            .fold(vec::Vec::<usize>::new(), |l, r| max_merge(&l, &r));
        distribute(&dimensions, self.config.width, self.config.padding)
    }

    fn fmt_row(&self,
               row: &[String],
               dimenstions: &[usize],
               f: &mut fmt::Formatter)
               -> fmt::Result {
        let expanded = dimenstions.iter()
            .zip(row.iter())
            .map(|(dim, cell)| split(cell, *dim))
            .collect::<Vec<_>>();
        let height = expanded.iter().map(|x| x.len()).max().unwrap_or(0);
        for i in 0..height {
            let row =
                join(expanded.iter()
                         .map(|x| {
                             x.get(i)
                                 .and_then(|x| Some(x.to_owned()))
                                 .unwrap_or_default()
                         })
                         .zip(dimenstions.iter())
                         .map(|(c, w)| {
                             format!("{pad}{cell: <width$}{pad}", pad = " ", width = w, cell = c)
                         }),
                     &self.config.border.0);
            try!(write!(f, "{}\n", row));
        }
        Ok(())
    }

    fn fmt_seperator(&self, dimensions: &[usize], f: &mut fmt::Formatter) -> fmt::Result {
        let row = join(dimensions.iter()
                           .map(|dim| {
                               iter::repeat(self.config.border.1.to_string())
                                   .take(dim + self.config.padding * 2)
                                   .collect::<String>()
                           }),
                       &self.config.border.2);
        write!(f, "{}\n", row)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dimensions = self.dimensions();
        if let Some(ref title) = self.title {
            try!(self.fmt_row(&title, &dimensions, f));
            try!(self.fmt_seperator(&dimensions, f));
        }
        for row in &self.rows {
            try!(self.fmt_row(row, &dimensions, f));
        }
        Ok(())
    }
}

fn split(cell: &str, w: usize) -> Vec<String> {
    let mut lines = vec![];
    let max = cell.len();
    let mut from = 0;
    while from < max {
        let till = cmp::min(from + w, max);
        let i = if till < max {
            match cell[from..till].rfind(' ') {
                Some(i) => i + 1,
                None => w,
            }
        } else {
            w
        };
        let till = cmp::min(from + i, max);
        lines.push(cell[from..till].trim().to_owned());
        from += i;
    }
    lines
}

fn flying(col_width: usize, cols: usize, width: usize, padding: usize) -> usize {
    let space = cols * 2 * padding + (cols - 1);
    let fair = (width - space) / cols;
    cmp::min(col_width, fair)
}

fn max_merge(left: &[usize], right: &[usize]) -> Vec<usize> {
    let mut merged =
        left.iter().zip(right.iter()).map(|(l, r)| *cmp::max(l, r)).collect::<Vec<_>>();
    let both = merged.len();
    merged.append(&mut left.iter().skip(both).cloned().collect::<Vec<_>>());
    merged.append(&mut right.iter().skip(both).cloned().collect::<Vec<_>>());
    merged
}

fn distribute(dimensions: &[usize], width: usize, padding: usize) -> Vec<usize> {
    let mut indexed = dimensions.iter().cloned().enumerate().collect::<Vec<_>>();
    indexed.sort_by(|a, b| a.1.cmp(&b.1));
    let mut width = width;
    let mut cols = dimensions.len();
    let mut distributed = indexed.iter()
        .map(|&(i, x)| {
            let size = flying(x, cols, width, padding);
            cols -= 1;
            if cols > 0 {
                width -= size + 2 * padding + 1;
            }
            (i, size)
        })
        .collect::<Vec<_>>();
    distributed.sort_by(|a, b| a.0.cmp(&b.0));
    distributed.iter().map(|&(_, x)| x).collect()
}


#[cfg(test)]
mod tests {
    macro_rules! ownv {
        ($($s:expr),*) => { vec!($($s.to_owned()), *) }
    }

    use super::*;
    #[test]
    fn it_works() {
        let mut table = Table::default();
        table.set_title(ownv!["who", "what"]);
        table.add_rows(&mut vec![ownv!["a", "b"], ownv!["c", "d"]]);
        table.add_row(ownv!["foobar", "foobar2000"]);
        assert_eq!(table.dimensions(), vec![6, 10]);
        let out = format!("{}", table);
        let should = "\
# who    | what       #
#--------+------------#
# a      | b          #
# c      | d          #
# foobar | foobar2000 #
"
            .replace("#", "");
        assert_eq!(out, should);
    }

    #[test]
    fn test_max_merge() {
        let l = vec![1, 2, 3];
        let r = vec![2, 0, 3, 4];
        assert_eq!(max_merge(&l, &r), vec![2, 2, 3, 4]);
        let l = vec![];
        let r = vec![2, 0, 3, 4];
        assert_eq!(max_merge(&l, &r), r);
    }

    #[test]
    fn test_split() {
        let cell = "foobar2000 foo";
        assert_eq!(split(cell, 12), ownv!("foobar2000", "foo"));
        let cell = "foobar2000    foo";
        assert_eq!(split(cell, 12), ownv!("foobar2000", "foo"));
        let cell = "foobar2000    foobar2000";
        assert_eq!(split(cell, 12), ownv!("foobar2000", "foobar2000"));
        let cell = "foobar2000     foobar2000";
        assert_eq!(split(cell, 12), ownv!("foobar2000", "", "foobar2000"));
    }

    #[test]
    fn test_distribute() {
        let dims = vec![10, 5, 20, 15];
        let dis = distribute(&dims, 40, 0);
        assert_eq!(dis, vec![10, 5, 11, 11]);
    }
    #[test]
    fn test_wrapping() {
        let mut table = Table::with_width(20);
        table.add_row(ownv!["da", "foobar foobar", "bar"]);
        table.add_row(ownv!["da", "foobar!", "bar"]);
        let out = format!("{}", table);
        let should = "\
# da | foobar  | bar #
#    | foobar  |     #
# da | foobar! | bar #
"
            .replace("#", "");
        assert_eq!(out, should);
    }

}
