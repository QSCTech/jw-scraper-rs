pub const SCORES_BASE_PAGE: &str = r##"

<!doctype HTML PUBLIC "-//W3C//DTD HTML 4.0 Transitional//EN">
<HTML>
  <HEAD>
		<title>浙江大学现代教务管理系统</title>
		<META HTTP-EQUIV="Content-Type" content="text/html; charset=gb2312">
		<META content="Microsoft Visual Studio .NET 7.0" name="GENERATOR">
		<META content="Visual Basic 7.0" name="CODE_LANGUAGE">
		<META content="JavaScript" name="vs_defaultClientScript">
		<META content="http://schemas.microsoft.com/intellisense/ie5" name="vs_targetSchema">
		<LINK rev="stylesheet" media="all" href="style/base.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/form.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/module.css" type="text/css" rel="stylesheet">
		<SCRIPT language="javascript">
            function window.onbeforeprint(){
                document.all.tabHidden.style.display = "none"
            }
            function window.onafterprint(){
                document.all.tabHidden.style.display = "block"
            }
            function click() {
                if (event.button==2) {  //改成button==2为禁止右键
                    alert('对不起,禁止使用此功能.')
                }
            }
            document.onmousedown=click
		</SCRIPT>
</HEAD>
	<body>
		<form name="Form1" method="post" action="xscj.aspx?xh=" id="Form1">
<input type="hidden" name="__VIEWSTATE" value="dDw0NzAzMzE4ODg7dDw7bDxpPDE+Oz47bDx0PDtsPGk8Mj47aTw1PjtpPDI1PjtpPDI3PjtpPDQxPjtpPDQzPjtpPDQ1PjtpPDQ3Pjs+O2w8dDx0PDt0PGk8MjA+O0A8XGU7MjAwMS0yMDAyOzIwMDItMjAwMzsyMDAzLTIwMDQ7MjAwNC0yMDA1OzIwMDUtMjAwNjsyMDA2LTIwMDc7MjAwNy0yMDA4OzIwMDgtMjAwOTsyMDA5LTIwMTA7MjAxMC0yMDExOzIwMTEtMjAxMjsyMDEyLTIwMTM7MjAxMy0yMDE0OzIwMTQtMjAxNTsyMDE1LTIwMTY7MjAxNi0yMDE3OzIwMTctMjAxODsyMDE4LTIwMTk7MjAxOS0yMDIwOz47QDxcZTsyMDAxLTIwMDI7MjAwMi0yMDAzOzIwMDMtMjAwNDsyMDA0LTIwMDU7MjAwNS0yMDA2OzIwMDYtMjAwNzsyMDA3LTIwMDg7MjAwOC0yMDA5OzIwMDktMjAxMDsyMDEwLTIwMTE7MjAxMS0yMDEyOzIwMTItMjAxMzsyMDEzLTIwMTQ7MjAxNC0yMDE1OzIwMTUtMjAxNjsyMDE2LTIwMTc7MjAxNy0yMDE4OzIwMTgtMjAxOTsyMDE5LTIwMjA7Pj47Pjs7Pjt0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8eHhxO3hxMTs+Pjs+O3Q8aTw4PjtAPFxlO+aYpTvlpI8755+tO+enizvlhqw755+tO+aakTs+O0A8XGU7MnzmmKU7MnzlpI87Mnznn607MXznp4s7MXzlhqw7MXznn607MXzmmpE7Pj47Pjs7Pjt0PHA8O3A8bDxvbmNsaWNrOz47bDx3aW5kb3cucHJpbnQoKVw7Oz4+Pjs7Pjt0PHA8O3A8bDxvbmNsaWNrOz47bDx3aW5kb3cuY2xvc2UoKVw7Oz4+Pjs7Pjt0PEAwPDs7Ozs7Ozs7Ozs+Ozs+O3Q8QDA8Ozs7Ozs7Ozs7Oz47Oz47dDxAMDw7Ozs7Ozs7Ozs7Pjs7Pjt0PHA8cDxsPFRleHQ7PjtsPFpKRFg7Pj47Pjs7Pjs+Pjs+Pjs+Q3rFGm8VZQ/qeumYsSX+AUiB9sk=" />

			<div class="indextop"></div>
			<div class="indexmid">
				<div class="toplogo">
					<h6></h6>
					<!-- <span><input type="text" value="输入帮助内容"> <a href="#"></a></span> -->
				</div>
				<div class="daohang">成绩查询</div>
				<div class="searchbar"></div>
				<div class="mainframe">
					<TABLE id="tabHidden" cellSpacing="0" cellPadding="3">
						<TR>
							<TD nowrap><span id="Label1">学年：</span><select name="ddlXN" id="ddlXN">
	<option value=""></option>
	<option value="2001-2002">2001-2002</option>
	<option value="2002-2003">2002-2003</option>
	<option value="2003-2004">2003-2004</option>
	<option value="2004-2005">2004-2005</option>
	<option value="2005-2006">2005-2006</option>
	<option value="2006-2007">2006-2007</option>
	<option value="2007-2008">2007-2008</option>
	<option value="2008-2009">2008-2009</option>
	<option value="2009-2010">2009-2010</option>
	<option value="2010-2011">2010-2011</option>
	<option value="2011-2012">2011-2012</option>
	<option value="2012-2013">2012-2013</option>
	<option value="2013-2014">2013-2014</option>
	<option value="2014-2015">2014-2015</option>
	<option value="2015-2016">2015-2016</option>
	<option value="2016-2017">2016-2017</option>
	<option value="2017-2018">2017-2018</option>
	<option value="2018-2019">2018-2019</option>
	<option value="2019-2020">2019-2020</option>

</select>
								<span id="Label2">学期：</span><select name="ddlXQ" id="ddlXQ">
	<option value=""></option>
	<option value="2|春">春</option>
	<option value="2|夏">夏</option>
	<option value="2|短">短</option>
	<option value="1|秋">秋</option>
	<option value="1|冬">冬</option>
	<option value="1|短">短</option>
	<option value="1|暑">暑</option>

</select>
								成绩段：<SPAN style="BORDER-BOTTOM:2px inset; BORDER-LEFT:2px inset; BORDER-TOP:2px inset; BORDER-RIGHT:2px inset"><input name="txtQSCJ" type="text" maxlength="3" id="txtQSCJ" onkeydown="if (window.event.keyCode==13) window.event.keyCode=9;" style="TEXT-ALIGN:right" />-<input name="txtZZCJ" type="text" maxlength="3" id="txtZZCJ" /></SPAN></TD>
						</TR>
						<TR>
							<TD>
								<input type="submit" name="Button1" value="按学期查询" id="Button1" class="aspbutton" />
								<input type="submit" name="Button5" value="按学年查询" id="Button5" class="aspbutton" />
								<input type="submit" name="Button2" value="在校学习成绩查询" id="Button2" class="aspbutton" />
								<input type="submit" name="Button6" value="学科讨论成绩查询" id="Button6" class="aspbutton" />
								<input type="submit" name="Button7" value="通识教育实践成绩查询" id="Button7" class="aspbutton" />
								<input type="submit" name="Button8" value="第二课堂及体测成绩查询" id="Button8" class="aspbutton" />
								<input type="submit" name="Button9" value="过程成绩查询" id="Button9" class="aspbutton" />
								<input type="submit" name="Button4" value="  打印  " id="Button4" class="aspbutton" onclick="window.print();" />
								<input type="submit" name="Button3" value="  关闭  " id="Button3" class="aspbutton" onclick="window.close();" /></TD>
							</TR>
					</TABLE>
					<BR>
					<TABLE id="Table1" cellSpacing="0" cellPadding="3" width="100%">
						<TR>
							<TD align="center" colspan="3"><span id="Label4"><b><font size="4"></font></b></span></TD>
						</TR>
						<TR>
							<TD><span id="Label3"></span></TD>
							<TD><span id="Label5"></span></TD>
							<TD><span id="Label6"></span></TD>
						</TR>
						<TR>
							<TD colSpan="2"><span id="Label9"></span></TD>
							<TD><span id="Label8"></span></TD>
						</TR>
					</TABLE>



					<TABLE width="100%">
						<TR>
							<TD align="right" colSpan="4">
								<img id="Image1" alt="学生条形码" border="0" /></TD>
						</TR>
					</TABLE>
				</div>
			</div>
			<div class="indexbot">浙江大学本科生院 版权所有</div>
		</form>
	</body>
</HTML>

"##;
