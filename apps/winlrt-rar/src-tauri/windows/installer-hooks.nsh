; winlrtRAR — Explorer sağ tık (Faz 3 beta)
; Kurulum sonrası SystemFileAssociations altına ek menüler (varsayılan uygulamayı değiştirmez).

!macro WinlrtArchiveVerb EXT KEY LABEL CMD
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" "${CMD}"
!macroend

!macro WinlrtArchiveVerbRemove EXT KEY
  DeleteRegKey SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}"
!macroend

!macro WinlrtDirectoryVerb KEY LABEL CMD
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" "${CMD}"
!macroend

!macro WinlrtDirectoryVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\Directory\shell\${KEY}"
!macroend

!macro WinlrtAllFilesVerb KEY LABEL CMD
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" "${CMD}"
!macroend

!macro WinlrtAllFilesVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\*\shell\${KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  !insertmacro WinlrtArchiveVerb "zip"  "WinlrtOpen"         "winlrtRAR ile aç"        '"$INSTDIR\winlrtRAR.exe" "%1"'
  !insertmacro WinlrtArchiveVerb "zip"  "WinlrtExtractHere"  "winlrtRAR — buraya aç"   '"$INSTDIR\winlrtRAR.exe" --extract-here "%1"'
  !insertmacro WinlrtArchiveVerb "wlrt" "WinlrtOpen"         "winlrtRAR ile aç"        '"$INSTDIR\winlrtRAR.exe" "%1"'
  !insertmacro WinlrtArchiveVerb "wlrt" "WinlrtExtractHere"  "winlrtRAR — buraya aç"   '"$INSTDIR\winlrtRAR.exe" --extract-here "%1"'
  !insertmacro WinlrtArchiveVerb "rar"  "WinlrtOpen"         "winlrtRAR ile aç"        '"$INSTDIR\winlrtRAR.exe" "%1"'
  !insertmacro WinlrtArchiveVerb "rar"  "WinlrtExtractHere"  "winlrtRAR — buraya aç"   '"$INSTDIR\winlrtRAR.exe" --extract-here "%1"'
  !insertmacro WinlrtArchiveVerb "7z"   "WinlrtOpen"         "winlrtRAR ile aç"        '"$INSTDIR\winlrtRAR.exe" "%1"'
  !insertmacro WinlrtArchiveVerb "7z"   "WinlrtExtractHere"  "winlrtRAR — buraya aç"   '"$INSTDIR\winlrtRAR.exe" --extract-here "%1"'
  !insertmacro WinlrtDirectoryVerb "WinlrtCompress"      "winlrtRAR ile sıkıştır"  '"$INSTDIR\winlrtRAR.exe" --compress-folder "%1"'
  !insertmacro WinlrtDirectoryVerb "WinlrtCompressSecure" "winlrtRAR — şifreli sıkıştır" '"$INSTDIR\winlrtRAR.exe" --compress-folder "%1" --secure'
  !insertmacro WinlrtDirectoryVerb "WinlrtCompressQuick" "winlrtRAR — hızlı .wlrt" '"$INSTDIR\winlrtRAR.exe" --compress-folder-quick "%1"'
  !insertmacro WinlrtAllFilesVerb "WinlrtCompressSelection" "winlrtRAR — seçilenleri sıkıştır" '"$INSTDIR\winlrtRAR.exe" --compress-selection %*'
  !insertmacro WinlrtAllFilesVerb "WinlrtCompressSelectionSecure" "winlrtRAR — seçilenler şifreli" '"$INSTDIR\winlrtRAR.exe" --compress-selection %* --secure'
  !insertmacro WinlrtAllFilesVerb "WinlrtCompressSelectionQuick" "winlrtRAR — seçilenler hızlı .wlrt" '"$INSTDIR\winlrtRAR.exe" --compress-selection-quick %*'
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  !insertmacro WinlrtArchiveVerbRemove "zip"  "WinlrtOpen"
  !insertmacro WinlrtArchiveVerbRemove "zip"  "WinlrtExtractHere"
  !insertmacro WinlrtArchiveVerbRemove "wlrt" "WinlrtOpen"
  !insertmacro WinlrtArchiveVerbRemove "wlrt" "WinlrtExtractHere"
  !insertmacro WinlrtArchiveVerbRemove "rar"  "WinlrtOpen"
  !insertmacro WinlrtArchiveVerbRemove "rar"  "WinlrtExtractHere"
  !insertmacro WinlrtArchiveVerbRemove "7z"   "WinlrtOpen"
  !insertmacro WinlrtArchiveVerbRemove "7z"   "WinlrtExtractHere"
  !insertmacro WinlrtDirectoryVerbRemove "WinlrtCompress"
  !insertmacro WinlrtDirectoryVerbRemove "WinlrtCompressSecure"
  !insertmacro WinlrtDirectoryVerbRemove "WinlrtCompressQuick"
  !insertmacro WinlrtAllFilesVerbRemove "WinlrtCompressSelection"
  !insertmacro WinlrtAllFilesVerbRemove "WinlrtCompressSelectionSecure"
  !insertmacro WinlrtAllFilesVerbRemove "WinlrtCompressSelectionQuick"
!macroend
