; Lertiva Arc — Explorer sağ tık (Faz 3 beta)

!macro WinlrtArchiveVerbOpen EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" "%1"'
!macroend

!macro WinlrtArchiveVerbExtract EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --extract-here "%1"'
!macroend

!macro WinlrtArchiveVerbRemove EXT KEY
  DeleteRegKey SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}"
!macroend

!macro WinlrtDirectoryVerbCompress KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-folder "%1"'
!macroend

!macro WinlrtDirectoryVerbCompressSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-folder "%1" --secure'
!macroend

!macro WinlrtDirectoryVerbCompressQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-folder-quick "%1"'
!macroend

!macro WinlrtDirectoryVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\Directory\shell\${KEY}"
!macroend

!macro WinlrtAllFilesVerbSelection KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-selection %*'
!macroend

!macro WinlrtAllFilesVerbSelectionSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-selection %* --secure'
!macroend

!macro WinlrtAllFilesVerbSelectionQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Arc.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Arc.exe" --compress-selection-quick %*'
!macroend

!macro WinlrtAllFilesVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\*\shell\${KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKLM "Software\winlrt\Lertiva Arc" "ProductVersion" "0.1.0"
  !insertmacro WinlrtArchiveVerbOpen "zip"  "WinlrtOpen"        "Lertiva Arc ile ac"
  !insertmacro WinlrtArchiveVerbExtract "zip"  "WinlrtExtractHere" "Lertiva Arc - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "wlrt" "WinlrtOpen"        "Lertiva Arc ile ac"
  !insertmacro WinlrtArchiveVerbExtract "wlrt" "WinlrtExtractHere" "Lertiva Arc - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "rar"  "WinlrtOpen"        "Lertiva Arc ile ac"
  !insertmacro WinlrtArchiveVerbExtract "rar"  "WinlrtExtractHere" "Lertiva Arc - buraya ac"
  !insertmacro WinlrtArchiveVerbOpen "7z"   "WinlrtOpen"        "Lertiva Arc ile ac"
  !insertmacro WinlrtArchiveVerbExtract "7z"   "WinlrtExtractHere" "Lertiva Arc - buraya ac"
  !insertmacro WinlrtDirectoryVerbCompress "WinlrtCompress" "Lertiva Arc ile sikistir"
  !insertmacro WinlrtDirectoryVerbCompressSecure "WinlrtCompressSecure" "Lertiva Arc - sifreli sikistir"
  !insertmacro WinlrtDirectoryVerbCompressQuick "WinlrtCompressQuick" "Lertiva Arc - hizli .wlrt"
  !insertmacro WinlrtAllFilesVerbSelection "WinlrtCompressSelection" "Lertiva Arc - secilenleri sikistir"
  !insertmacro WinlrtAllFilesVerbSelectionSecure "WinlrtCompressSelectionSecure" "Lertiva Arc - secilenler sifreli"
  !insertmacro WinlrtAllFilesVerbSelectionQuick "WinlrtCompressSelectionQuick" "Lertiva Arc - secilenler hizli .wlrt"
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
