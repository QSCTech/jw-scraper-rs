use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SchoolYear(u16);

#[derive(Debug)]
pub enum CourseSemester {
    FallAndWinter,
    SpringAndSummer,
}

#[derive(Debug)]
pub enum ExamSemester {
    SummerHoliday,
    Fall,
    Winter,
    WinterShort,
    Spring,
    Summer,
    SummerShort,
}

impl Display for SchoolYear {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("{}-{}", self.0, self.0 + 1))
    }
}

use CourseSemester::*;
impl Deref for CourseSemester {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            FallAndWinter => "1|秋、冬",
            SpringAndSummer => "2|春、夏",
        }
    }
}

use ExamSemester::*;
impl Deref for ExamSemester {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            SummerHoliday => "1|暑",
            Fall => "1|秋",
            Winter => "1|冬",
            WinterShort => "1|短",
            Spring => "2|春",
            Summer => "2|夏",
            SummerShort => "2|短",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_school_year_to_string() {
        assert_eq!("2017-2018", &SchoolYear(2017).to_string())
    }

    #[test]
    fn test_course_semester_as_ref() {
        let semester: &str = &FallAndWinter;
        assert_eq!("1|秋、冬", semester)
    }

    #[test]
    fn test_exam_semester_as_ref() {
        let semester: &str = &Fall;
        assert_eq!("1|秋", semester)
    }
}
