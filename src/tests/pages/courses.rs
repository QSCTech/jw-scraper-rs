pub const COURSES_PAGE: &str = r##"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.0 Transitional//EN">
<HTML>
	<HEAD>
		<TITLE>浙江大学现代教务管理系统</TITLE>
		<META http-equiv="Content-Type" content="text/html; charset=gb2312">
		<META content="Microsoft Visual Studio .NET 7.0" name="GENERATOR">
		<META content="Visual Basic 7.0" name="CODE_LANGUAGE">
		<META content="JavaScript" name="vs_defaultClientScript">
		<META content="http://schemas.microsoft.com/intellisense/ie5" name="vs_targetSchema">
		<LINK rev="stylesheet" media="all" href="style/base.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/form.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/module.css" type="text/css" rel="stylesheet">
		<link rev="stylesheet" media="print" href="style/print.css" type="text/css" rel="stylesheet">
		<style type="text/css">
		.transparent { BORDER-RIGHT: indianred 1px solid; BORDER-TOP: indianred 1px solid; DISPLAY: none; FILTER: alpha(opacity=85); BORDER-LEFT: indianred 1px solid; BORDER-BOTTOM: indianred 1px solid; POSITION: absolute; BACKGROUND-COLOR: infobackground }
		a img{ border:none; }
		</style>
		<SCRIPT language="javascript">
		    function ShowWin(sURL){
		        window.open(sURL);
		    }
		    	function Hide(){
				Popup.style.display="none";
				}

		    	function show(xzxh){
		    	var sksj;
		    	if (xzxh=="1"){
		    	  sksj="8:00-8:45";
		    	}else if (xzxh=="2"){
		    	 sksj="8:50-9:35";
		    	}else if (xzxh=="3"){
		    	 sksj="9:50-10:35";
		    	}else if (xzxh=="4"){
		    	 sksj="10:40-11:25";
		    	}else if (xzxh=="5"){
		    	 sksj="11:30-12:15";
		    	}else if (xzxh=="6"){
		    	 sksj="13:15-14:00";
		    	}else if (xzxh=="7"){
		    	 sksj="14:05-14:50";
		    	}else if (xzxh=="8"){
		    	 sksj="14:55-15:40";
		    	}else if (xzxh=="9"){
		    	 sksj="15:55-16:40";
		    	}else if (xzxh=="10"){
		    	 sksj="16:45-17:30";
		    	}else if (xzxh=="11"){
		    	 sksj="18:30-19:15";
		    	}else if (xzxh=="12"){
		    	 sksj="19:20-20:05";
		    	}else if (xzxh=="13"){
		    	 sksj="20:10-20:55";
		    	}
				document.getElementById("td1").innerHTML=sksj;
				x=event.clientX + document.body.scrollLeft;
				y=event.clientY + document.body.scrollTop;
				Popup.style.display="block";
				Popup.style.left = x;
				Popup.style.top = y;
				}
		</SCRIPT>
		<style>

		.daohang .mingzi { FONT-WEIGHT: bold; FONT-SIZE: 14px; COLOR: #223285 }

		</style>
	</HEAD>
	<body>
		<form name="xskb_form" method="post" action="xskbcx.aspx?xh=" id="xskb_form">
<input type="hidden" name="__EVENTTARGET" value="" />
<input type="hidden" name="__EVENTARGUMENT" value="" />
<input type="hidden" name="__VIEWSTATE" value="dDwtMjQ5Nzk5MzUyO3Q8O2w8aTwwPjs+O2w8dDw7bDxpPDE+O2k8Mz47aTw1PjtpPDg+O2k8MTA+O2k8MTI+O2k8MTQ+O2k8MTY+O2k8MTg+O2k8MjI+O2k8MjY+O2k8Mjg+Oz47bDx0PHQ8OztsPGk8MD47Pj47Oz47dDx0PHA8cDxsPERhdGFUZXh0RmllbGQ7RGF0YVZhbHVlRmllbGQ7PjtsPHhuO3huOz4+Oz47dDxpPDM+O0A8MjAxOC0yMDE5OzIwMTctMjAxODsyMDE2LTIwMTc7PjtAPDIwMTgtMjAxOTsyMDE3LTIwMTg7MjAxNi0yMDE3Oz4+O2w8aTwwPjs+Pjs7Pjt0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8ZHl4cTt4cTE7Pj47Pjt0PGk8Mj47QDzmmKXjgIHlpI8756eL44CB5YasOz47QDwyfOaYpeOAgeWkjzsxfOeni+OAgeWGrDs+PjtsPGk8MD47Pj47Oz47dDxwPHA8bDxUZXh0Oz47bDzlrablj7fvvJozMTYwMTAxMDM0Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlp5PlkI3vvJrmnY7mmajmm6Y7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWtpumZou+8mua1t+a0i+WtpumZojs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857G7KOS4k+S4minvvJrmtbfmtIvlt6XnqIvkuI7mioDmnK87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOihjOaUv+ePre+8mua1t+a0i+W3peeoi+S4juaKgOacrzE2MDI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFxlOz4+Oz47Oz47dDxAMDxwPHA8bDxWaXNpYmxlO1BhZ2VDb3VudDtfIUl0ZW1Db3VudDtfIURhdGFTb3VyY2VJdGVtQ291bnQ7RGF0YUtleXM7PjtsPG88dD47aTwxPjtpPDg+O2k8OD47bDw+Oz4+Oz47Ozs7Ozs7Ozs7PjtsPGk8MD47PjtsPHQ8O2w8aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3PjtpPDg+Oz47bDx0PDtsPGk8MD47aTwxPjtpPDI+O2k8Mz47aTw0PjtpPDU+O2k8Nj47aTw3Pjs+O2w8dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wMTFMMDEwMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPjAxMUwwMTAwXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wMTFMMDEwMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuS4reWbvee7j+a1jueDreeCueino+ivu1w8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDExTDAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mma/kuYPmnYNcPC9hXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOaYpTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85ZGo5LiJ56ysNiw3LDjoioI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+S4nDFBLTIxNijlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA1OjE2OjQxOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDQxSTAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4wNDFJMDEwMFw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDQxSTAxMDAzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7kuJbnlYzlkI3or5fmrKPotY9cPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwMTAwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5ZC056ybXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKU7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOS4ieesrDExLDEyLDEz6IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8yLTIwMyjlpJopIzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxNTowMjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwNDQwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MDQxSTA0NDBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA0MUkwNDQwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5Lit5Zu95Y+k5YW45paH5a2m5qyj6LWPXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wNDFJMDQ0MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWtmeaVj+W8ulw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pilOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuoznrKwxMSwxMiwxM+iKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS00MDIo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxNTo1Nzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA2MUI5MDkwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MDYxQjkwOTBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA2MUI5MDkwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5qaC546H6K665LiO5pWw55CG57uf6K6hXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0wNjFCOTA5MDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuWQtOWbveaholw8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pil5aSPOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuIDnrKw26IqCXDxiclw+5ZGo5LiA56ysNyw46IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8xLTIwOCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA0OjMzOjM1Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDgxQzAxOTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4wODFDMDE5MVw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMDgxQzAxOTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mnLrmorDorr7orqHln7rnoYDvvIjnlLLvvIlcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTA4MUMwMTkxMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+566h5oiQXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKXlpI87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOS4gOesrDMsNCw16IqCOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzntKvph5HmuK/opb8xLTQwNCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE4LTEyLTI3IDA1OjE5OjUwOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47dDw7bDxpPDA+O2k8MT47aTwyPjtpPDM+O2k8ND47aTw1PjtpPDY+O2k8Nz47PjtsPHQ8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMTAxQzAyNTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD4xMDFDMDI1MVw8L0FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8XDxBIGhyZWY9JyMnIG9uY2xpY2s9IndpbmRvdy5vcGVuKCd4c3hqcy5hc3B4P3hra2g9VCgyMDE4LTIwMTktMiktMTAxQzAyNTEzMTYwMTAxMDM0Jywna2NiJywndG9vbGJhcj0wLGxvY2F0aW9uPTAsZGlyZWN0b3JpZXM9MCxzdGF0dXM9MCxtZW51YmFyPTAsc2Nyb2xsYmFycz0xLHJlc2l6YWJsZT0xJykiXD7mlbDlrZfnlLXot6/liIbmnpDkuI7orr7orqFcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTEwMUMwMjUxMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5ZGo566tXDwvYVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzmmKXlpI87Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOWRqOWbm+esrDYsNyw4LDnoioJ75Y+M5ZGofVw8YnJcPuWRqOWbm+esrDYsNyw46IqCe+WNleWRqH07Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+S4nDMtMzA2XDxiclw+57Sr6YeR5riv6KW/MS0zMTQo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxMjowMTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTIwMUwwMDMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+MjAxTDAwMzBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTIwMUwwMDMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5biC5Zy66JCl6ZSA5qaC6K66XDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS0yMDFMMDAzMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPuW+kOW/oOa1t1w8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85aSPOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuIDnrKwxMSwxMiwxM+iKgjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w857Sr6YeR5riv6KW/MS0xMDMo5aSaKTs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MjAxOC0xMi0yNyAwNToxODo0Mzs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w8MTs+Pjs+Ozs+Oz4+O3Q8O2w8aTwwPjtpPDE+O2k8Mj47aTwzPjtpPDQ+O2k8NT47aTw2PjtpPDc+Oz47bDx0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTY4MTkwMTMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+NjgxOTAxMzBcPC9BXD47Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFw8QSBocmVmPScjJyBvbmNsaWNrPSJ3aW5kb3cub3BlbigneHN4anMuYXNweD94a2toPVQoMjAxOC0yMDE5LTIpLTY4MTkwMTMwMzE2MDEwMTAzNCcsJ2tjYicsJ3Rvb2xiYXI9MCxsb2NhdGlvbj0wLGRpcmVjdG9yaWVzPTAsc3RhdHVzPTAsbWVudWJhcj0wLHNjcm9sbGJhcnM9MSxyZXNpemFibGU9MScpIlw+5L+h5Y+35LiO57O757ufXDwvQVw+Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDxcPEEgaHJlZj0nIycgb25jbGljaz0id2luZG93Lm9wZW4oJ3hzeGpzLmFzcHg/eGtraD1UKDIwMTgtMjAxOS0yKS02ODE5MDEzMDMxNjAxMDEwMzQnLCdrY2InLCd0b29sYmFyPTAsbG9jYXRpb249MCxkaXJlY3Rvcmllcz0wLHN0YXR1cz0wLG1lbnViYXI9MCxzY3JvbGxiYXJzPTEscmVzaXphYmxlPTEnKSJcPui1teWdh1w8L2FcPjs+Pjs+Ozs+O3Q8cDxwPGw8VGV4dDs+O2w85pilOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDzlkajkuoznrKw3LDjoioJcPGJyXD7lkajlm5vnrKwxLDLoioI7Pj47Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPOe0q+mHkea4r+ilvzEtNTA0KOWkmilcPGJyXD7ntKvph5HmuK/opb8xLTUwNCjlpJopOz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwyMDE5LTAxLTA2IDAwOjUzOjE2Oz4+Oz47Oz47dDxwPHA8bDxUZXh0Oz47bDwxOz4+Oz47Oz47Pj47Pj47Pj47dDxAMDxwPHA8bDxQYWdlQ291bnQ7XyFJdGVtQ291bnQ7XyFEYXRhU291cmNlSXRlbUNvdW50O0RhdGFLZXlzOz47bDxpPDE+O2k8MD47aTwwPjtsPD47Pj47Pjs7Ozs7Ozs7Ozs+Ozs+O3Q8O2w8aTwzPjs+O2w8dDxAMDw7Ozs7Ozs7Ozs7Pjs7Pjs+Pjs+Pjs+Pjs+Alal0Mdy1EQJ38XRwfsObXeLLBw=" />

<script language="javascript" type="text/javascript">
<!--
	function __doPostBack(eventTarget, eventArgument) {
		var theform;
		if (window.navigator.appName.toLowerCase().indexOf("microsoft") > -1) {
			theform = document.xskb_form;
		}
		else {
			theform = document.forms["xskb_form"];
		}
		theform.__EVENTTARGET.value = eventTarget.split("$").join(":");
		theform.__EVENTARGUMENT.value = eventArgument;
		theform.submit();
	}
// -->
</script>

			<FONT face="宋体"></FONT><FONT face="宋体"></FONT>
			<div class="indextop"></div>
			<div class="indexmid">
				<div class="toplogo">
					<h6></h6>
				</div>
				<div class="daohang" nowrap>
					<table style="MARGIN:3px 0px 0px 2em">
						<tr>
							<td class="mingzi">个人课表查询</td>
							<td>（显示模式</td>
							<td><table id="xxms" border="0">
	<tr>
		<td><input id="xxms_0" type="radio" name="xxms" value="列表" checked="checked" onclick="__doPostBack('xxms_0','')" language="javascript" /><label for="xxms_0">列表模式</label></td><td><input id="xxms_1" type="radio" name="xxms" value="表格" onclick="__doPostBack('xxms_1','')" language="javascript" /><label for="xxms_1">表格模式</label></td>
	</tr>
</table></td>
							<td>）</td>
						</tr>
					</table>
				</div>
				<div class="mainframe">
					<TABLE id="Table2" cellSpacing="0" cellPadding="3" width="650">
						<TR>
							<TD align="center"><select name="xnd" onchange="__doPostBack('xnd','')" language="javascript" id="xnd">
	<option selected="selected" value="2018-2019">2018-2019</option>
	<option value="2017-2018">2017-2018</option>
	<option value="2016-2017">2016-2017</option>

</select><span id="Label2"><font face="宋体" size="5">学年</font></span><select name="xqd" onchange="__doPostBack('xqd','')" language="javascript" id="xqd">
	<option selected="selected" value="2|春、夏">春、夏</option>
	<option value="1|秋、冬">秋、冬</option>

</select><span id="Label1"><font face="宋体" size="5">学期学生个人课程表</font></span></TD>
						</TR>
					</TABLE>
					<TABLE id="Table13" cellSpacing="0" cellPadding="3" width="650">
						<TR>
							<TD width="33%"><span id="Label5">学号：</span></TD>
							<TD width="33%"><span id="Label6">姓名：</span></TD>
							<TD width="34%"><span id="Label7">学院：</span></TD>
						</TR>
					</TABLE>
					<TABLE id="Table11" cellSpacing="0" cellPadding="3" width="650">
						<TR>
							<TD width="50%"><span id="Label8">类(专业)：</span></TD>
							<TD width="50%"><span id="Label9">行政班：</span></TD>
						</TR>
					</TABLE>
					<span id="L_bz"></span>

					<table class="datagridstyle" cellspacing="0" cellpadding="3" rules="all" border="1" id="xsgrid" width="100%">
	<tr class="datagridhead">
		<td>课程代码</td><td>课程名称</td><td>教师姓名</td><td>学期</td><td>上课时间</td><td>上课地点</td><td>选课时间</td><td>选课志愿</td>
	</tr><tr>
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-011L0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">011L0100</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-011L0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">中国经济热点解读</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-011L0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">景乃权</a></td><td>春</td><td>周三第6,7,8节</td><td>紫金港东1A-216(多)</td><td>2018-12-27 05:16:41</td><td>1</td>
	</tr><tr class="datagrid1212">
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">041I0100</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">世界名诗欣赏</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0100','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">吴笛</a></td><td>春</td><td>周三第11,12,13节</td><td>紫金港西2-203(多)#</td><td>2018-12-27 05:15:02</td><td>1</td>
	</tr><tr>
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0440','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">041I0440</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0440','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">中国古典文学欣赏</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-041I0440','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">孙敏强</a></td><td>春</td><td>周二第11,12,13节</td><td>紫金港西1-402(多)</td><td>2018-12-27 05:15:57</td><td>1</td>
	</tr><tr class="datagrid1212">
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-061B9090','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">061B9090</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-061B9090','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">概率论与数理统计</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-061B9090','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">吴国桢</a></td><td>春夏</td><td>周一第6节<br>周一第7,8节</td><td>紫金港西1-208(多)</td><td>2018-12-27 04:33:35</td><td>1</td>
	</tr><tr>
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-081C0191','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">081C0191</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-081C0191','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">机械设计基础（甲）</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-081C0191','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">管成</a></td><td>春夏</td><td>周一第3,4,5节</td><td>紫金港西1-404(多)</td><td>2018-12-27 05:19:50</td><td>1</td>
	</tr><tr class="datagrid1212">
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-101C0251','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">101C0251</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-101C0251','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">数字电路分析与设计</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-101C0251','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">周箭</a></td><td>春夏</td><td>周四第6,7,8,9节{双周}<br>周四第6,7,8节{单周}</td><td>紫金港东3-306<br>紫金港西1-314(多)</td><td>2018-12-27 05:12:01</td><td>1</td>
	</tr><tr>
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-201L0030','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">201L0030</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-201L0030','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">市场营销概论</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-201L0030','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">徐忠海</a></td><td>夏</td><td>周一第11,12,13节</td><td>紫金港西1-103(多)</td><td>2018-12-27 05:18:43</td><td>1</td>
	</tr><tr class="datagrid1212">
		<td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-68190130','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">68190130</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-68190130','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">信号与系统</A></td><td><A href='#' onclick="window.open('xsxjs.aspx?xkkh=T(2018-2019-2)-68190130','kcb','toolbar=0,location=0,directories=0,status=0,menubar=0,scrollbars=1,resizable=1')">赵均</a></td><td>春</td><td>周二第7,8节<br>周四第1,2节</td><td>紫金港西1-504(多)<br>紫金港西1-504(多)</td><td>2019-01-06 00:53:16</td><td>1</td>
	</tr>
</table>
					<fieldset><legend><span id="Label_yxjxb">选择服从教学班调配</span></legend><table class="datagridstyle" cellspacing="0" cellpadding="3" rules="all" border="1" id="Datagrid_fcjxb" width="100%">
	<tr class="datagridhead">
		<td>课程代码</td><td>课程名称</td><td>调配结果</td><td>选择时间</td>
	</tr>
</table></fieldset>

					<input name="kcxx" type="text" onchange="__doPostBack('kcxx','')" language="javascript" id="kcxx" /></FONT>
				</div>
				课表显示说明：确定的课程用<font color="blue">蓝色</font>显示，待定的课程用<i><font color="red">红色斜体</font></i>显示。
			</div>
			<div class="indexbot">浙江大学本科生院 版权所有</div>
			<div class="transparent" id="Popup">
				<table style="FONT-SIZE: x-small" cellPadding="0" border="0">
					<tr>
						<td align="center"><FONT face="宋体" color="blue">上课时间</FONT></td>
					</tr>
					<tr>
						<td id="td1"><FONT face="宋体"></FONT></td>
					</tr>
				</table>
			</div>
		</form>
<!--飘窗-->

		<script type="text/javascript" src="js/wesad.js"></script>
<script type="text/javascript">
var mm1;
function mm1() {
	checkbrOK();
	mm1=new Chip("mm1",0,0);
	if(brOK) {
		movechip("mm1");
	}
}
</script>
<div id="mm1" style="position:absolute; left:14px; top:337px; cursor:hand;" onmouseout="movechip('mm1')" onmouseover="stopme('mm1')">
	<DIV onclick="hidead(1);" style="FONT-SIZE: 9pt;position: absolute;top:0px;right:0px;margin:2px;padding:6px;z-index:2000; cursor:hand;" align=right>关闭×</DIV>
	<a href="xsmain_pyjh_tctz.aspx" target="_blank" ><img src="images/piaochuang/piaochuang3.jpg"/></a>
</div>
	<script type="text/javascript">
	if ('2016'=="2015"){
		mm1();
               //hidead(1);
	}
	else
	{
		hidead(1);
	}

	</script>

	</body>
</HTML>

"##;
