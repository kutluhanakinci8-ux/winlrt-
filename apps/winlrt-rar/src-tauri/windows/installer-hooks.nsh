; winlrtRAR — Explorer sağ tık (Faz 3 beta)

!macro WinlrtArchiveVerbOpen EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" "%1"'
!macroend

!macro WinlrtArchiveVerbExtract EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --extract-here "%1"'
!macroend

!macro WinlrtArchiveVerbRemove EXT KEY
  DeleteRegKey SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}"
!macroend

!macro WinlrtDirectoryVerbCompress KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-folder "%1"'
!macroend

!macro WinlrtDirectoryVerbCompressSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-folder "%1" --secure'
!macroend

!macro WinlrtDirectoryVerbCompressQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-folder-quick "%1"'
!macroend

!macro WinlrtDirectoryVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\Directory\shell\${KEY}"
!macroend

!macro WinlrtAllFilesVerbSelection KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-selection %*'
!macroend

!macro WinlrtAllFilesVerbSelectionSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-selection %* --secure'
!macroend

!macro WinlrtAllFilesVerbSelectionQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtRAR.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\winlrtRAR.exe" --compress-selection-quick %*'
!macroend

!macro WinlrtAllFilesVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\*\shell\${KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKLM "Software\winlrt\winlrtRAR" "ProductVersion" "0.1.0"
  !insertmacro WinlrtArchiveVerbOpen "zip"  "WinlrtOpen"        "winlrtRAR ile ac"
  !insertmacro WinlrtArchiveVerbExtract "zip"  "WinlrtExtractHere" "winlrtRAR - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "wlrt" "WinlrtOpen"        "winlrtRAR ile ac"
  !insertmacro WinlrtArchiveVerbExtract "wlrt" "WinlrtExtractHere" "winlrtRAR - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "rar"  "WinlrtOpen"        "winlrtRAR ile ac"
  !insertmacro WinlrtArchiveVerbExtract "rar"  "WinlrtExtractHere" "winlrtRAR - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "7z"   "WinlrtOpen"        "winlrtRAR ile ac"
  !insertmacro WinlrtArchiveVerbExtract "7z"   "WinlrtExtractHere" "winlrtRAR - buraya ac"
  !insertmacro WinlrtDirectoryVerbCompress "WinlrtCompress" "winlrtRAR ile sikistir"
  !insertmacro WinlrtDirectoryVerbCompressSecure "WinlrtCompressSecure" "winlrtRAR - sifreli sikistir"
  !insertmacro WinlrtDirectoryVerbCompressQuick "WinlrtCompressQuick" "winlrtRAR - hizli .wlrt"
  !insertmacro WinlrtAllFilesVerbSelection "WinlrtCompressSelection" "winlrtRAR - secilenleri sikistir"
  !insertmacro WinlrtAllFilesVerbSelectionSecure "WinlrtCompressSelectionSecure" "winlrtRAR - secilenler sifreli"
  !insertmacro WinlrtAllFilesVerbSelectionQuick "WinlrtCompressSelectionQuick" "winlrtRAR - secilenler hizli .wlrt"
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
