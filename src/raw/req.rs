use interfacer_http::ToContent;
use serde::{Deserialize, Serialize};

pub const LOGIN_VIEW_STATE: &str = "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=";
pub const DEFAULT_COURSES_VIEW_STATE: &str = "dDwtMjQ5Nzk5MzUyO3Q8O2w8aTwwPjs+O2w8dDw7bDxpPDE+O2k8Mz47aTw1PjtpPDg+O2k8MTA+O2k8MTI+O2k8MTQ+O2k8MTY+O2k8MTg+O2k8MjI+O2k8MjY+O2k8Mjg+Oz47bDx0PHQ8OztsPGk8MD47Pj47Oz47dDx0PHA8cDxsPERhdGFUZXh0RmllbGQ7RGF0YVZhbHVlRmllbGQ7PjtsPHhuO3huOz4+Oz47dDxpPDM+O0A8MjAxOC0yMDE5OzIwMTctMjAxODsyMDE2LTIwMTc7PjtAPDIwMTgtMjAxOTsyMDE3LTIwMTg7MjAxNi0yMDE3Oz4+O2w8aTwwPjs+Pjs7Pjt0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8ZHl4cTt4cTE7Pj47Pjt0PGk8Mj47QDznp4vjgIHlhqw75pil44CB5aSPOz47QDwxfOeni+OAgeWGrDsyfOaYpeOAgeWkjzs+PjtsPGk8MD47Pj47Oz47dDxwPHA8bDxUZXh0Oz47bDzlrablj7fvvJozMTYwMTAxMDM0Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlp5PlkI3vvJrmnY7mmajmm6Y7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWtpumZou+8mua1t+a0i+WtpumZojs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857G7KOS4k+S4minvvJrmtbfmtIvlt6XnqIvkuI7mioDmnK87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOihjOaUv+ePre+8mua1t+a0i+W3peeoi+S4juaKgOacrzE2MDI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFxlOz4+Oz47Oz47dDxAMDxwPHA8bDxWaXNpYmxlO1BhZ2VDb3VudDtfIUl0ZW1Db3VudDtfIURhdGFTb3VyY2VJdGVtQ291bnQ7RGF0YUtleXM7PjtsPG88dD47aTwxPjtpPDEwPjtpPDEwPjtsPD47Pj47Pjs7Ozs7Ozs7Ozs+O2w8aTwwPjs+O2w8dDw7bDxpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+O2k8OD47aTw5PjtpPDEwPjs+O2w8dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9MygyMDE4LTIwMTktMSktMDUxRjA2MDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4wNTFGMDYwMFw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9MygyMDE4LTIwMTktMSktMDUxRjA2MDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7oi7Hor63msLTlubPmtYvor5VcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPTMoMjAxOC0yMDE5LTEpLTA1MUYwNjAwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5pa55a+M5rCRXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDznp4vlhqw7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPCZuYnNwXDs7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPCZuYnNwXDs7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDIwMTgtMTEtMjQgMDE6MTQ6NDM7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDE7Pj47Pjs7Pjs+Pjt0PDtsPGk8MD47aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3Pjs+O2w8dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0wNjFCMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPjA2MUIwMDIwXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0wNjFCMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWkjeWPmOWHveaVsOS4juenr+WIhuWPmOaNolw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktMDYxQjAwMjAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7njovkvJ9cPC9hXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOenizs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85ZGo5LiA56ysMSwy6IqCXDxiclw+5ZGo5Zub56ysMSwy6IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8yLTIwNSjlpJopXDxiclw+57Sr6YeR5riv6KW/Mi0yMDUo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0wNi0xMyAxNDo0ODo1Mjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTEwMUMwMzUwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MTAxQzAzNTBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTEwMUMwMzUwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+55S16Lev5LiO5qih5ouf55S15a2Q5oqA5pyvXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0xMDFDMDM1MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWtmeebvlw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w856eL5YasOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuoznrKw2LDfoioJcPGJyXD7lkajkuoznrKw46IqCe+WNleWRqH1cPGJyXD7lkajkupTnrKwzLDQsNeiKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS00MTco5aSaKVw8YnJcPue0q+mHkea4r+ilvzEtNDE3KOWkmilcPGJyXD7ntKvph5HmuK/opb8xLTQxNyjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTA2LTEzIDE0OjQ5OjM1Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktMTAxQzAzNjAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4xMDFDMDM2MFw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktMTAxQzAzNjAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7nlLXot6/kuI7mqKHmi5/nlLXlrZDmioDmnK/lrp7pqoxcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTEwMUMwMzYwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5bmy5LqOXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDznp4vlhqw7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOWbm+esrDMsNCw16IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/kuJwzLTIwMjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0wNi0xMyAxNDo1MjowMDs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTI0MUwwMDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MjQxTDAwMjBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTI0MUwwMDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5Y2a5byI6K665Z+656GAXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0yNDFMMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuiSi+aWh+WNjlw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85YasOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuInnrKw2LDcsOOiKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS0zMTYo5aSaKSo7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDIwMTgtMDYtMTMgMTU6MTk6MDg7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDE7Pj47Pjs7Pjs+Pjt0PDtsPGk8MD47aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3Pjs+O2w8dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0yNjFDMDA3MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPjI2MUMwMDcwXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS0yNjFDMDA3MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuW3peeoi+WKm+Wtplw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktMjYxQzAwNzAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7lkLTnprlcPGJyXD7lraPokYbljY5cPC9hXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOeni+WGrDs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85ZGo5LqM56ysMSwy6IqCe+WNleWRqH1cPGJyXD7lkajlm5vnrKw2LDfoioJcPGJyXD7lkajlm5vnrKw46IqCe+WPjOWRqH07Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+ilvzEtNDA0KOWkmilcPGJyXD7ntKvph5HmuK/opb8xLTQwNCjlpJopXDxiclw+57Sr6YeR5riv6KW/MS00MDQo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0wNi0xMyAxNDo1NToxOTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTc0MTg4MDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+NzQxODgwMjBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTc0MTg4MDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5LiT5Lia5a6e5LmgXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS03NDE4ODAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPumZiOWutuaXulw8YnJcPum7hOixquW9qVw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w855+tOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwmbmJzcFw7Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwmbmJzcFw7Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTA2LTEzIDIwOjA3OjM0Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktNzYxVDAwMTAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD43NjFUMDAxMFw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktNzYxVDAwMTAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7lpKflrabniannkIbvvIjnlLLvvInihaBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTc2MVQwMDEwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5r2Y5Zu95Y2rXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDznp4vlhqw7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOWFreesrDYsNyw4LDnoioI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+ilvzItMTAxKOWkmik7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDIwMTgtMDktMTQgMTM6MDM6MTU7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPDE7Pj47Pjs7Pjs+Pjt0PDtsPGk8MD47aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3Pjs+O2w8dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS03NjFUMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPjc2MVQwMDIwXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS03NjFUMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWkp+WtpueJqeeQhu+8iOeUsu+8ieKFoVw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMSktNzYxVDAwMjAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7pg5HlpKfmlrlcPC9hXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOeni+WGrDs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85ZGo5LiA56ysMyw06IqCXDxiclw+5ZGo5LiJ56ysMSwy6IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8yLTIwMijlpJopIzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0wNi0xMyAxNDo0MzowMzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTgyMVQwMDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+ODIxVDAwMjBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTEpLTgyMVQwMDIwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5b6u56ev5YiG77yI55Sy77yJ4oWhXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0xKS04MjFUMDAyMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuiWm+WEkuiLsVw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w856eL5YasOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajlha3nrKwxLDIsMyw06IqCe+WNleWRqH1cPGJyXD7lkajlha3nrKwxLDIsMyw0LDXoioJ75Y+M5ZGofTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/Mi0xMDUo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0wOS0xNCAxMzowMTo0Nzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+Oz4+Oz4+O3Q8QDA8cDxwPGw8UGFnZUNvdW50O18hSXRlbUNvdW50O18hRGF0YVNvdXJjZUl0ZW1Db3VudDtEYXRhS2V5czs+O2w8aTwxPjtpPDA+O2k8MD47bDw+Oz4+Oz47Ozs7Ozs7Ozs7Pjs7Pjt0PDtsPGk8Mz47PjtsPHQ8QDA8Ozs7Ozs7Ozs7Oz47Oz47Pj47Pj47Pj47PpKFTtRexRJ7a2Y+N2+LD/9F//0R";
pub const COURSES_EVENT_TARGET: &str = "xqd";
pub const DEFAULT_EXAMS_VIEW_STATE: &str = "dDwxNTk1Njg3ODE4O3Q8O2w8aTwxPjs+O2w8dDw7bDxpPDE+O2k8NT47PjtsPHQ8dDxwPHA8bDxEYXRhVGV4dEZpZWxkO0RhdGFWYWx1ZUZpZWxkOz47bDx4bjt4bjs+Pjs+O3Q8aTwzPjtAPDIwMTgtMjAxOTsyMDE3LTIwMTg7MjAxNi0yMDE3Oz47QDwyMDE4LTIwMTk7MjAxNy0yMDE4OzIwMTYtMjAxNzs+PjtsPGk8MD47Pj47Oz47dDx0PHA8cDxsPERhdGFUZXh0RmllbGQ7RGF0YVZhbHVlRmllbGQ7PjtsPHh4cTt4cTE7Pj47Pjt0PGk8Nz47QDzmmKU75aSPO+efrTvnp4s75YasO+efrTvmmpE7PjtAPDJ85pilOzJ85aSPOzJ855+tOzF856eLOzF85YasOzF855+tOzF85pqROz4+O2w8aTwwPjs+Pjs7Pjs+Pjs+Pjs+OCzbPClMQn2a0gVUtI8dCTWFDFY=";

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct LoginBody<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "TextBox1")]
    username: &'a str,

    #[serde(rename = "TextBox2")]
    password: &'a str,

    #[serde(rename = "RadioButtonList1")]
    radio_button: &'a str,

    #[serde(rename = "Text1")]
    text1: &'a str,
}

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct CoursesReq<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "xxms")]
    repr_mode: &'a str,

    #[serde(rename = "xnd")]
    school_year: &'a str,

    #[serde(rename = "xqd")]
    semester: &'a str,

    #[serde(rename = "kcxx")]
    course_into: &'a str,
}

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct ExamsReq<'a> {
    #[serde(rename = "__EVENTTARGET")]
    event_target: &'a str,

    #[serde(rename = "__EVENTARGUMENT")]
    event_argument: &'a str,

    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "xnd")]
    school_year: &'a str,

    #[serde(rename = "xqd")]
    semester: &'a str,
}

#[derive(Serialize, Deserialize, ToContent, Debug)]
pub struct ScoresReq<'a> {
    #[serde(rename = "__VIEWSTATE")]
    view_state: &'a str,

    #[serde(rename = "ddlXN")]
    school_year: &'a str,

    #[serde(rename = "ddlXQ")]
    semester: &'a str,

    #[serde(rename = "txtQSCJ")]
    lower_bound: &'a str,

    #[serde(rename = "txtZZCJ")]
    upper_bound: &'a str,

    #[serde(rename = "Button2")]
    search_button: &'a str,
}

impl<'a> LoginBody<'a> {
    pub fn new(view_state: &'a str, username: &'a str, password: &'a str) -> Self {
        Self {
            event_target: "Button1",
            event_argument: "",
            view_state,
            username,
            password,
            radio_button: "学生",
            text1: "",
        }
    }
}

impl<'a> CoursesReq<'a> {
    pub fn new(
        event_target: &'a str,
        view_state: &'a str,
        school_year: &'a str,
        semester: &'a str,
    ) -> Self {
        Self {
            event_target,
            event_argument: "",
            view_state,
            repr_mode: "列表",
            school_year,
            semester,
            course_into: "",
        }
    }
}

impl<'a> ExamsReq<'a> {
    pub fn new(
        event_target: &'a str,
        view_state: &'a str,
        school_year: &'a str,
        semester: &'a str,
    ) -> Self {
        Self {
            event_target,
            event_argument: "",
            view_state,
            school_year,
            semester,
        }
    }
}

impl<'a> ScoresReq<'a> {
    pub fn new(view_state: &'a str) -> Self {
        Self {
            view_state,
            school_year: "",
            semester: "",
            lower_bound: "",
            upper_bound: "",
            search_button: "在校学习成绩查询",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interfacer_http::{
        ToContent,
    };

    #[test]
    fn login_body() {
        let body = LoginBody::new(
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "3160100000",
            "123456",
        );
        let data: Vec<u8> = body
            .to_content(&"application/x-www-form-urlencoded; charset=gb2312".parse().unwrap())
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=Button1&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&TextBox1=3160100000&TextBox2=123456&RadioButtonList1=%D1%A7%C9%FA&Text1=",
            &String::from_utf8(data).unwrap()
        );
    }

    #[test]
    fn courses_req() {
        let req = CoursesReq::new(
            "xnd",
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "2018-2019",
            "1|秋、冬",
        );
        let data: Vec<u8> = req
            .to_content(&"application/x-www-form-urlencoded; charset=gb2312".parse().unwrap())
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=xnd&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&xxms=%C1%D0%B1%ED&xnd=2018-2019&xqd=1%7C%C7%EF%A1%A2%B6%AC&kcxx=",
            &String::from_utf8(data).unwrap()
        );
    }

    #[test]
    fn exams_req() {
        let req = ExamsReq::new(
            "xnd",
            "dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=",
            "2018-2019",
            "2|春",
        );
        let data: Vec<u8> = req
            .to_content(&"application/x-www-form-urlencoded; charset=gb2312".parse().unwrap())
            .unwrap();

        assert_eq!(
            "__EVENTTARGET=xnd&__EVENTARGUMENT=&__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&xnd=2018-2019&xqd=2%7C%B4%BA",
            &String::from_utf8(data).unwrap()
        );
    }

    #[test]
    fn scores_req() {
        let req = ScoresReq::new("dDwxNTc0MzA5MTU4Ozs+b5wKASjiu+fSjITNzcKuKXEUyXg=");
        let data: Vec<u8> = req
            .to_content(&"application/x-www-form-urlencoded; charset=gb2312".parse().unwrap())
            .unwrap();

        assert_eq!(
            "__VIEWSTATE=dDwxNTc0MzA5MTU4Ozs%2Bb5wKASjiu%2BfSjITNzcKuKXEUyXg%3D&ddlXN=&ddlXQ=&txtQSCJ=&txtZZCJ=&Button2=%D4%DA%D0%A3%D1%A7%CF%B0%B3%C9%BC%A8%B2%E9%D1%AF",
            &String::from_utf8(data).unwrap()
        );
    }
}
