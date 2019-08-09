pub const EXAMS_PAGE: &str = r##"

<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.0 Transitional//EN">
<HTML>
	<HEAD>
		<title>浙江大学现代教务管理系统</title>
		<meta HTTP-EQUIV="Content-Type" content="text/html; charset=gb2312">
		<meta name="GENERATOR" content="Microsoft Visual Studio .NET 7.0">
		<meta name="CODE_LANGUAGE" content="Visual Basic 7.0">
		<meta name="vs_defaultClientScript" content="JavaScript">
		<meta name="vs_targetSchema" content="http://schemas.microsoft.com/intellisense/ie5">
		<LINK rev="stylesheet" media="all" href="style/base.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/form.css" type="text/css" rel="stylesheet">
		<LINK rev="stylesheet" media="all" href="style/module.css" type="text/css" rel="stylesheet">
	</HEAD>
	<body>
		<form name="_ctl0" method="post" action="xskscx.aspx?xh=" id="_ctl0">
<input type="hidden" name="__EVENTTARGET" value="" />
<input type="hidden" name="__EVENTARGUMENT" value="" />
<input type="hidden" name="__VIEWSTATE" value="dDwtMTIwNDIxMTMxOTt0PDtsPGk8MT47PjtsPHQ8O2w8aTwxPjtpPDU+Oz47bDx0PHQ8cDxwPGw8RGF0YVRleHRGaWVsZDtEYXRhVmFsdWVGaWVsZDs+O2w8eG47eG47Pj47Pjt0PGk8Mz47QDwyMDE4LTIwMTk7MjAxNy0yMDE4OzIwMTYtMjAxNzs+O0A8MjAxOC0yMDE5OzIwMTctMjAxODsyMDE2LTIwMTc7Pj47bDxpPDA+Oz4+Ozs+O3Q8dDxwPHA8bDxEYXRhVGV4dEZpZWxkO0RhdGFWYWx1ZUZpZWxkOz47bDx4eHE7eHExOz4+Oz47dDxpPDc+O0A856eLO+WGrDvnn6075pqRO+aYpTvlpI8755+tOz47QDwxfOenizsxfOWGrDsxfOefrTsxfOaakTsyfOaYpTsyfOWkjzsyfOefrTs+PjtsPGk8MD47Pj47Oz47Pj47Pj47PhxzNOFHvHmAawHKYWrnuScM29Kw" />

<script language="javascript" type="text/javascript">
<!--
	function __doPostBack(eventTarget, eventArgument) {
		var theform;
		if (window.navigator.appName.toLowerCase().indexOf("microsoft") > -1) {
			theform = document._ctl0;
		}
		else {
			theform = document.forms["_ctl0"];
		}
		theform.__EVENTTARGET.value = eventTarget.split("$").join(":");
		theform.__EVENTARGUMENT.value = eventArgument;
		theform.submit();
	}
// -->
</script>

			<div class="indextop"></div>
			<div class="indexmid">
				<div class="toplogo">
					<h6></h6>

				</div>
				<div class="daohang">学生考试查询</div>
				<div class="mainframe">



			<TABLE id="Table2" cellSpacing="0" cellPadding="0" width="100%">
				<TR>
					<TD align="center">
						<select name="xnd" onchange="__doPostBack('xnd','')" language="javascript" id="xnd">
	<option selected="selected" value="2018-2019">2018-2019</option>
	<option value="2017-2018">2017-2018</option>
	<option value="2016-2017">2016-2017</option>

</select>
						<span id="Label2"><font face="宋体" size="5">学年第</font></span>
						<select name="xqd" onchange="__doPostBack('xqd','')" language="javascript" id="xqd">
	<option selected="selected" value="1|秋">秋</option>
	<option value="1|冬">冬</option>
	<option value="1|短">短</option>
	<option value="1|暑">暑</option>
	<option value="2|春">春</option>
	<option value="2|夏">夏</option>
	<option value="2|短">短</option>

</select><span id="Label1"><font face="宋体" size="5">学期学生考试查询</font></span></TD>
				</TR>
			</TABLE>
			<hr>
			<table class="datagridstyle" cellspacing="0" cellpadding="3" border="0" id="DataGrid1" width="100%">
	<tr class="datagridhead">
		<td>选课课号</td><td>课程名称</td><td>学分</td><td>重修标记</td><td>姓名</td><td>学期</td><td>期末考试时间</td><td>期末考试地点</td><td>期末考试座位号</td><td>期中考试时间</td><td>期中考试地点</td><td>期中考试座位号</td>
	</tr><tr>
		<td>(2018-2019-1)-061B9090</td><td>概率论与数理统计[补考]</td><td>2.5</td><td>&nbsp;</td><td></td><td>秋</td><td>2018年09月13日(14:00-16:00)</td><td>紫金港西2-102(多)</td><td>133</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td>
	</tr><tr class="datagrid1212">
		<td>(2018-2019-1)-061B0020-0094312-1</td><td>复变函数与积分变换</td><td>1.5</td><td>1</td><td></td><td>秋</td><td>2018年11月15日(08:00-10:00)</td><td>紫金港西2-202(多)#</td><td>29</td><td>&nbsp;</td><td>&nbsp;</td><td>&nbsp;</td>
	</tr>
</table></div>
			</div>
			<div class="indexbot">浙江大学本科生院 版权所有</div>
		</form>
	</body>
</HTML>

"##;
