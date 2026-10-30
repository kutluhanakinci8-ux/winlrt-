; winlrtZIP — Explorer sag tik (W4)

!macro WinlrtZipVerbOpen EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" "%1"'
!macroend

!macro WinlrtZipVerbExtract EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --extract-here "%1"'
!macroend

!macro WinlrtZipVerbRemove EXT KEY
  DeleteRegKey SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}"
!macroend

!macro WinlrtZipDirectoryVerbCompress KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --compress-folder "%1"'
!macroend

!macro WinlrtZipDirectoryVerbCompressQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --compress-folder-quick "%1"'
!macroend

!macro WinlrtZipDirectoryVerbCompressSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --compress-folder "%1" --secure'
!macroend

!macro WinlrtZipDirectoryVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\Directory\shell\${KEY}"
!macroend

!macro WinlrtZipAllFilesVerbSelection KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --compress-selection %*'
!macroend

!macro WinlrtZipAllFilesVerbSelectionSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\winlrtZIP.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\winlrtZIP.exe" --compress-selection %* --secure'
!macroend

!macro WinlrtZipAllFilesVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\*\shell\${KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKLM "Software\winlrt\winlrtZIP" "ProductVersion" "0.1.0"
  !insertmacro WinlrtZipVerbOpen "zip" "WinlrtZipOpen" "winlrtZIP ile ac"
  !insertmacro WinlrtZipVerbExtract "zip" "WinlrtZipExtractHere" "winlrtZIP - buraya ac"
  !insertmacro WinlrtZipDirectoryVerbCompress "WinlrtZipCompress" "winlrtZIP ile sikistir"
  !insertmacro WinlrtZipDirectoryVerbCompressQuick "WinlrtZipCompressQuick" "winlrtZIP - hizli ZIP"
  !insertmacro WinlrtZipDirectoryVerbCompressSecure "WinlrtZipCompressSecure" "winlrtZIP - sifreli sikistir"
  !insertmacro WinlrtZipAllFilesVerbSelection "WinlrtZipCompressSelection" "winlrtZIP - secilenleri sikistir"
  !insertmacro WinlrtZipAllFilesVerbSelectionSecure "WinlrtZipCompressSelectionSecure" "winlrtZIP - secilenler sifreli"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  !insertmacro WinlrtZipVerbRemove "zip" "WinlrtZipOpen"
  !insertmacro WinlrtZipVerbRemove "zip" "WinlrtZipExtractHere"
  !insertmacro WinlrtZipDirectoryVerbRemove "WinlrtZipCompress"
  !insertmacro WinlrtZipDirectoryVerbRemove "WinlrtZipCompressQuick"
  !insertmacro WinlrtZipDirectoryVerbRemove "WinlrtZipCompressSecure"
  !insertmacro WinlrtZipAllFilesVerbRemove "WinlrtZipCompressSelection"
  !insertmacro WinlrtZipAllFilesVerbRemove "WinlrtZipCompressSelectionSecure"
!macroend
