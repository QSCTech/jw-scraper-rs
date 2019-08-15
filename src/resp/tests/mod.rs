mod pages;

use self::pages::{
    COURSES_PAGE, COURSE_INFO, EXAMS_PAGE, LOGIN_PAGE, MAJOR_SCORES_PAGE, OBJECT_MOVED_PAGE,
    SCORES_BASE_PAGE, SCORES_PAGE, TOTAL_CREDIT_PAGE,
};
use super::ExamTime;
use super::{
    CoursesPage, ExamsPage, LoginPage, MajorScore, MajorScoresPage, MajorSummaryTable,
    ObjectMovedPage, ScoresBasePage, ScoresPage, TotalCreditPage,
};
use reformation::Reformation;
use unhtml::FromHtml;

#[test]
fn login() {
    let page = LoginPage::from_html(LOGIN_PAGE).unwrap();
    assert_eq!(
        "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
        &page.hidden_form.view_state
    );
    assert_eq!("", &page.hidden_form.event_target);
    assert_eq!("", &page.hidden_form.event_argument);
}

#[test]
fn courses() {
    let page = CoursesPage::from_html(COURSES_PAGE).unwrap();
    assert_eq!(
        "dDwtMjQ5Nzk5MzUyO3Q8O2w8aTwwPjs+O2w8dDw7bDxpPDE+O2k8Mz47aTw1PjtpPDg+O2k8MTA+O2k8MTI+O2k8MTQ+O2k8MTY+O2k8MTg+O2k8MjI+O2k8MjY+O2k8Mjg+Oz47bDx0PHQ8OztsPGk8MD47Pj47Oz47dDx0PHA8cDxsPERhdGFUZXh0RmllbGQ7RGF0YVZhbHVlRmllbGQ7PjtsPHhuO3huOz4+Oz47dDxpPDM+O0A8MjAxOC0yMDE5OzIwMTctMjAxODsyMDE2LTIwMTc7PjtAPDIwMTgtMjAxOTsyMDE3LTIwMTg7MjAxNi0yMDE3Oz4+O2w8aTwwPjs+Pjs7Pjt0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8ZHl4cTt4cTE7Pj47Pjt0PGk8Mj47QDzmmKXjgIHlpI8756eL44CB5YasOz47QDwyfOaYpeOAgeWkjzsxfOeni+OAgeWGrDs+PjtsPGk8MD47Pj47Oz47dDxwPHA8bDxUZXh0Oz47bDzlrablj7fvvJozMTYwMTAxMDM0Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlp5PlkI3vvJrmnY7mmajmm6Y7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWtpumZou+8mua1t+a0i+WtpumZojs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857G7KOS4k+S4minvvJrmtbfmtIvlt6XnqIvkuI7mioDmnK87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOihjOaUv+ePre+8mua1t+a0i+W3peeoi+S4juaKgOacrzE2MDI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFxlOz4+Oz47Oz47dDxAMDxwPHA8bDxWaXNpYmxlO1BhZ2VDb3VudDtfIUl0ZW1Db3VudDtfIURhdGFTb3VyY2VJdGVtQ291bnQ7RGF0YUtleXM7PjtsPG88dD47aTwxPjtpPDg+O2k8OD47bDw+Oz4+Oz47Ozs7Ozs7Ozs7PjtsPGk8MD47PjtsPHQ8O2w8aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3PjtpPDg+Oz47bDx0PDtsPGk8MD47aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3Pjs+O2w8dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wMTFMMDEwMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPjAxMUwwMTAwXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wMTFMMDEwMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuS4reWbvee7j+a1jueDreeCueino+ivu1w8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDExTDAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mma/kuYPmnYNcPC9hXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOaYpTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85ZGo5LiJ56ysNiw3LDjoioI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+S4nDFBLTIxNijlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA1OjE2OjQxOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDQxSTAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4wNDFJMDEwMFw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDQxSTAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7kuJbnlYzlkI3or5fmrKPotY9cPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwMTAwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5ZC056ybXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKU7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOS4ieesrDExLDEyLDEz6IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8yLTIwMyjlpJopIzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxNTowMjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwNDQwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MDQxSTA0NDBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwNDQwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5Lit5Zu95Y+k5YW45paH5a2m5qyj6LWPXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wNDFJMDQ0MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWtmeaVj+W8ulw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pilOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuoznrKwxMSwxMiwxM+iKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS00MDIo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxNTo1Nzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA2MUI5MDkwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MDYxQjkwOTBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA2MUI5MDkwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5qaC546H6K665LiO5pWw55CG57uf6K6hXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wNjFCOTA5MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWQtOWbveaholw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pil5aSPOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuIDnrKw26IqCXDxiclw+5ZGo5LiA56ysNyw46IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8xLTIwOCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA0OjMzOjM1Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDgxQzAxOTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4wODFDMDE5MVw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDgxQzAxOTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mnLrmorDorr7orqHln7rnoYDvvIjnlLLvvIlcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA4MUMwMTkxMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+566h5oiQXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKXlpI87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOS4gOesrDMsNCw16IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8xLTQwNCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA1OjE5OjUwOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMTAxQzAyNTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4xMDFDMDI1MVw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMTAxQzAyNTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mlbDlrZfnlLXot6/liIbmnpDkuI7orr7orqFcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTEwMUMwMjUxMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5ZGo566tXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKXlpI87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOWbm+esrDYsNyw4LDnoioJ75Y+M5ZGofVw8YnJcPuWRqOWbm+esrDYsNyw46IqCe+WNleWRqH07Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+S4nDMtMzA2XDxiclw+57Sr6YeR5riv6KW/MS0zMTQo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxMjowMTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTIwMUwwMDMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MjAxTDAwMzBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTIwMUwwMDMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5biC5Zy66JCl6ZSA5qaC6K66XDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0yMDFMMDAzMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuW+kOW/oOa1t1w8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85aSPOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuIDnrKwxMSwxMiwxM+iKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS0xMDMo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxODo0Mzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTY4MTkwMTMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+NjgxOTAxMzBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTY4MTkwMTMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5L+h5Y+35LiO57O757ufXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS02ODE5MDEzMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPui1teWdh1w8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pilOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuoznrKw3LDjoioJcPGJyXD7lkajlm5vnrKwxLDLoioI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+ilvzEtNTA0KOWkmilcPGJyXD7ntKvph5HmuK/opb8xLTUwNCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE5LTAxLTA2IDAwOjUzOjE2Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47Pj47Pj47dDxAMDxwPHA8bDxQYWdlQ291bnQ7XyFJdGVtQ291bnQ7XyFEYXRhU291cmNlSXRlbUNvdW50O0RhdGFLZXlzOz47bDxpPDE+O2k8MD47aTwwPjtsPD47Pj47Pjs7Ozs7Ozs7Ozs+Ozs+O3Q8O2w8aTwzPjs+O2w8dDxAMDw7Ozs7Ozs7Ozs7Pjs7Pjs+Pjs+Pjs+Pjs+Alal0Mdy1EQJ38XRwfsObXeLLBw=",
        &page.hidden_form.view_state,
    );

    assert_eq!("2018-2019", &page.school_year.selected.unwrap());
    assert_eq!(3, page.school_year.all_options.len());
    assert_eq!("2018-2019", &page.school_year.all_options[0]);
    assert_eq!("2017-2018", &page.school_year.all_options[1]);
    assert_eq!("2016-2017", &page.school_year.all_options[2]);
    assert_eq!("2|春、夏", &page.semester.selected.unwrap());
    assert_eq!(2, page.semester.all_options.len());
    assert_eq!("2|春、夏", &page.semester.all_options[0]);
    assert_eq!("1|秋、冬", &page.semester.all_options[1]);
    assert_eq!(8, page.courses.len());
    assert_eq!("011L0100", &page.courses[0].code);
    assert_eq!("68190130", &page.courses[7].code);
}

#[test]
fn exams() {
    let page = ExamsPage::from_html(EXAMS_PAGE).unwrap();
    assert_eq!(
        "dDwtMTIwNDIxMTMxOTt0PDtsPGk8MT47PjtsPHQ8O2w8aTwxPjtpPDU+Oz47bDx0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8eG47eG47Pj47Pjt0PGk8Mz47QDwyMDE4LTIwMTk7MjAxNy0yMDE4OzIwMTYtMjAxNzs+O0A8MjAxOC0yMDE5OzIwMTctMjAxODsyMDE2LTIwMTc7Pj47bDxpPDA+Oz4+Ozs+O3Q8dDxwPHA8bDxEYXRhVGV4dEZpZWxkO0RhdGFWYWx1ZUZpZWxkOz47bDx4eHE7eHExOz4+Oz47dDxpPDc+O0A856eLO+WGrDvnn6075pqRO+aYpTvlpI8755+tOz47QDwxfOenizsxfOWGrDsxfOefrTsxfOaakTsyfOaYpTsyfOWkjzsyfOefrTs+PjtsPGk8MD47Pj47Oz47Pj47Pj47PhxzNOFHvHmAawHKYWrnuScM29Kw",
        &page.hidden_form.view_state,
    );
    assert_eq!("2018-2019", &page.school_year.selected.unwrap());
    assert_eq!(3, page.school_year.all_options.len());
    assert_eq!("2018-2019", &page.school_year.all_options[0]);
    assert_eq!("2017-2018", &page.school_year.all_options[1]);
    assert_eq!("2016-2017", &page.school_year.all_options[2]);
    assert_eq!("1|秋", &page.semester.selected.unwrap());
    assert_eq!(7, page.semester.all_options.len());
    assert_eq!("1|秋", &page.semester.all_options[0]);
    assert_eq!("1|冬", &page.semester.all_options[1]);
    assert_eq!("1|短", &page.semester.all_options[2]);
    assert_eq!("1|暑", &page.semester.all_options[3]);
    assert_eq!("2|春", &page.semester.all_options[4]);
    assert_eq!("2|夏", &page.semester.all_options[5]);
    assert_eq!("2|短", &page.semester.all_options[6]);
    assert_eq!(2, page.exams.len());
    assert_eq!("(2018-2019-1)-061B9090", &page.exams[0].identifier);
    assert_eq!(
        "(2018-2019-1)-061B0020-0094312-1",
        &page.exams[1].identifier
    );
}

#[test]
fn exam_time_regex() {
    let time = ExamTime::parse("2019年01月13日(08:00-10:00)").unwrap();
    assert_eq!(2019, time.year);
    assert_eq!(1, time.month);
    assert_eq!(13, time.day);
    assert_eq!(8, time.start_hour);
    assert_eq!(0, time.start_min);
    assert_eq!(10, time.end_hour);
    assert_eq!(0, time.end_min);
}

#[test]
fn object_moved_page() {
    let page = ObjectMovedPage::from_html(OBJECT_MOVED_PAGE).unwrap();
    assert_eq!("/dcwj.aspx?xh=3160100000", &page.to)
}

#[test]
fn major_summary_table() {
    let MajorSummaryTable { gpa, total_credit } =
        "主修专业课程累计平均绩点=2.25&nbsp;&nbsp;&nbsp;&nbsp;主修专业课程累计获得总学分=58.00"
            .parse()
            .unwrap();
    assert_eq!(2.25, gpa);
    assert_eq!(58.00, total_credit);
}

#[test]
fn major_score() {
    let score = MajorScore::from_html("
    <table><tr><td>(2016-2017-2)-211G0210-0098253-1</td><td>C程序设计</td><td>85</td><td>85</td><td>3.0</td><td>3.90</td><td>2016-2017</td></tr></table>").unwrap();
    assert_eq!("(2016-2017-2)-211G0210-0098253-1", &score.identifier);
    assert_eq!("C程序设计", &score.course_name);
    assert_eq!("85", &score.raw_score);
    assert_eq!(85f32, score.final_score);
    assert_eq!(3.0f32, score.credit);
    assert_eq!(3.90f32, score.grade_point);
    assert_eq!("2016-2017", &score.school_year);
}

#[test]
fn major_scores_page() {
    let MajorScoresPage {
        scores,
        summary_table,
    } = MajorScoresPage::from_html(MAJOR_SCORES_PAGE).unwrap();
    let MajorSummaryTable { gpa, total_credit } = summary_table;
    assert_eq!(2.25, gpa);
    assert_eq!(58.00, total_credit);
    assert_eq!(35, scores.len());
}

#[test]
fn scores_page() {
    let ScoresPage { scores } = ScoresPage::from_html(SCORES_PAGE).unwrap();
    assert_eq!(23, scores.len());
    let score = &scores[0];
    assert_eq!("(2016-2017-1)-021E0010-0092466-5", &score.identifier);
    assert_eq!("思想道德修养与法律基础", &score.course_name);
    assert_eq!("83", &score.raw_score);
    assert_eq!(2.5f32, score.credit);
    assert_eq!(3.90f32, score.grade_point);
    assert_eq!("", &score.makeup_score);
}

#[test]
fn scores_base_page() {
    let ScoresBasePage {
        hidden_form,
        school_year,
    } = ScoresBasePage::from_html(SCORES_BASE_PAGE).unwrap();
    assert_eq!(
        "dDw0NzAzMzE4ODg7dDw7bDxpPDE+Oz47bDx0PDtsPGk8Mj47aTw1PjtpPDI1PjtpPDI3PjtpPDQxPjtpPDQzPjtpPDQ1PjtpPDQ3Pjs+O2w8dDx0PDt0PGk8MjA+O0A8XGU7MjAwMS0yMDAyOzIwMDItMjAwMzsyMDAzLTIwMDQ7MjAwNC0yMDA1OzIwMDUtMjAwNjsyMDA2LTIwMDc7MjAwNy0yMDA4OzIwMDgtMjAwOTsyMDA5LTIwMTA7MjAxMC0yMDExOzIwMTEtMjAxMjsyMDEyLTIwMTM7MjAxMy0yMDE0OzIwMTQtMjAxNTsyMDE1LTIwMTY7MjAxNi0yMDE3OzIwMTctMjAxODsyMDE4LTIwMTk7MjAxOS0yMDIwOz47QDxcZTsyMDAxLTIwMDI7MjAwMi0yMDAzOzIwMDMtMjAwNDsyMDA0LTIwMDU7MjAwNS0yMDA2OzIwMDYtMjAwNzsyMDA3LTIwMDg7MjAwOC0yMDA5OzIwMDktMjAxMDsyMDEwLTIwMTE7MjAxMS0yMDEyOzIwMTItMjAxMzsyMDEzLTIwMTQ7MjAxNC0yMDE1OzIwMTUtMjAxNjsyMDE2LTIwMTc7MjAxNy0yMDE4OzIwMTgtMjAxOTsyMDE5LTIwMjA7Pj47Pjs7Pjt0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8eHhxO3hxMTs+Pjs+O3Q8aTw4PjtAPFxlO+aYpTvlpI8755+tO+enizvlhqw755+tO+aakTs+O0A8XGU7MnzmmKU7MnzlpI87Mnznn607MXznp4s7MXzlhqw7MXznn607MXzmmpE7Pj47Pjs7Pjt0PHA8O3A8bDxvbmNsaWNrOz47bDx3aW5kb3cucHJpbnQoKVw7Oz4+Pjs7Pjt0PHA8O3A8bDxvbmNsaWNrOz47bDx3aW5kb3cuY2xvc2UoKVw7Oz4+Pjs7Pjt0PEAwPDs7Ozs7Ozs7Ozs+Ozs+O3Q8QDA8Ozs7Ozs7Ozs7Oz47Oz47dDxAMDw7Ozs7Ozs7Ozs7Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFpKRFg7Pj47Pjs7Pjs+Pjs+Pjs+Q3rFGm8VZQ/qeumYsSX+AUiB9sk=",
        hidden_form.view_state
    );
    assert_eq!(None, school_year.selected);
    assert_eq!(20, school_year.all_options.len());
}

#[test]
fn total_credit_page() {
    let TotalCreditPage { credit } = TotalCreditPage::from_html(TOTAL_CREDIT_PAGE).unwrap();
    assert_eq!(83f32, credit);
}
