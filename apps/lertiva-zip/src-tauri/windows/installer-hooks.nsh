; Lertiva Zip — Explorer sag tik (W4)

!macro WinlrtZipVerbOpen EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" "%1"'
!macroend

!macro WinlrtZipVerbExtract EXT KEY LABEL
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --extract-here "%1"'
!macroend

!macro WinlrtZipVerbRemove EXT KEY
  DeleteRegKey SHCTX "Software\Classes\SystemFileAssociations\.${EXT}\shell\${KEY}"
!macroend

!macro WinlrtZipDirectoryVerbCompress KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --compress-folder "%1"'
!macroend

!macro WinlrtZipDirectoryVerbCompressQuick KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --compress-folder-quick "%1"'
!macroend

!macro WinlrtZipDirectoryVerbCompressSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\Directory\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --compress-folder "%1" --secure'
!macroend

!macro WinlrtZipDirectoryVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\Directory\shell\${KEY}"
!macroend

!macro WinlrtZipAllFilesVerbSelection KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --compress-selection %*'
!macroend

!macro WinlrtZipAllFilesVerbSelectionSecure KEY LABEL
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "" "${LABEL}"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "Icon" "$INSTDIR\Lertiva Zip.exe,0"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}" "MultiSelectModel" "Document"
  WriteRegStr SHCTX "Software\Classes\*\shell\${KEY}\command" "" '"$INSTDIR\Lertiva Zip.exe" --compress-selection %* --secure'
!macroend

!macro WinlrtZipAllFilesVerbRemove KEY
  DeleteRegKey SHCTX "Software\Classes\*\shell\${KEY}"
!macroend

!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr HKLM "Software\Lertiva\Lertiva Zip" "ProductVersion" "0.1.0"
  !insertmacro WinlrtZipVerbOpen "zip" "WinlrtZipOpen" "Lertiva Zip ile ac"
  !insertmacro WinlrtZipVerbExtract "zip" "WinlrtZipExtractHere" "Lertiva Zip - buraya ac"
  !insertmacro WinlrtZipDirectoryVerbCompress "WinlrtZipCompress" "Lertiva Zip ile sikistir"
  !insertmacro WinlrtZipDirectoryVerbCompressQuick "WinlrtZipCompressQuick" "Lertiva Zip - hizli ZIP"
  !insertmacro WinlrtZipDirectoryVerbCompressSecure "WinlrtZipCompressSecure" "Lertiva Zip - sifreli sikistir"
  !insertmacro WinlrtZipAllFilesVerbSelection "WinlrtZipCompressSelection" "Lertiva Zip - secilenleri sikistir"
  !insertmacro WinlrtZipAllFilesVerbSelectionSecure "WinlrtZipCompressSelectionSecure" "Lertiva Zip - secilenler sifreli"
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
