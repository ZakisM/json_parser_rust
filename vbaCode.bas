Option Explicit

Declare PtrSafe Function get_key Lib "C:\Users\zak\Code\json_parser_rust\target\x86_64-pc-windows-msvc\debug\json_parser_rust.dll" (ByVal inputText As LongPtr, ByVal inputLength As Long) As String
Declare PtrSafe Sub SysFreeString Lib "oleaut32.dll" (ByVal bstr As LongPtr)

Sub Call_Rust()
	Dim text As String
	text = "abc2123xyz"

	Dim result As String
	result = StrConv(get_key(StrPtr(text), Len(text)), vbFromUnicode)

	Debug.Print result
End Sub
