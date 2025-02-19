message("-- nostr_client_relay static build: " ${BUILD_STATIC})
message(STATUS "Source directory is " ${CMAKE_SOURCE_DIR})
message(STATUS "Build directory is " ${CMAKE_CURRENT_BINARY_DIR})

#//////////////////////////
# GUI WxWidgets, gnostro
# /home/pvn/git/nostr_client_relay/build/ext/wxWidgets-3.2.2.1/wx-config --libs
# wx-config --cppflags
#//////////////////////////

if (LINUX)
  add_definitions(-D__WXGTK3__)
  add_definitions(-D__WXGTK__)
  link_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib)
  include_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib/wx/include/gtk3-unicode-static-3.2)
endif()

if (MSVC)
  include_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib/vc_x64_lib/mswud)
  link_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib/vc_x64_lib)
elseif(APPLE)
  include_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib/wx/include/osx_cocoa-unicode-static-3.2)
  link_directories(${CMAKE_BINARY_DIR}/ext/wxWidgets-3.2.2.1/lib)
  add_definitions(-DWXUSINGDLL -D__WXOSX_COCOA__ -D__WXMAC__ -D__WXOSX__)
endif()

include_directories(${CMAKE_SOURCE_DIR}/ext/wxWidgets-3.2.2.1/include)

#//////////////////////////
# gnostro sources
#//////////////////////////

set(src_extra ${src_extra})
set(src_extra ${src_extra} ${CMAKE_SOURCE_DIR}/desktop/gnostro.hh)
set(src_extra ${src_extra} ${CMAKE_SOURCE_DIR}/desktop/gnostro.cc)

if (MSVC)
  set(src_extra ${src_extra} ${CMAKE_SOURCE_DIR}/desktop/sample.rc)
elseif(APPLE AND NOT IPHONE)
  set(src_extra ${src_extra} ${CMAKE_SOURCE_DIR}/ext/wxWidgets-3.2.2.1/src/osx/carbon/wxmac.icns)
endif()

#//////////////////////////
# gnostro executable
#//////////////////////////

add_executable(gnostro WIN32 MACOSX_BUNDLE ${src_extra})

#//////////////////////////
# gnostro link
#//////////////////////////

if (MSVC)
  set(lib_dep ${lib_dep} wxbase32ud wxmsw32ud_core wxmsw32ud_webview wxbase32ud_net wxpngd wxzlibd wxregexud wxmsw32ud_html)
  set(lib_dep ${lib_dep} wxscintillad wxmsw32ud_stc)
  set(lib_dep ${lib_dep} odbc32.lib odbccp32.lib comctl32.lib rpcrt4.lib ws2_32.lib)
endif()

if (LINUX)
  set(lib_dep ${lib_dep} wx_gtk3u_xrc-3.2 wx_gtk3u_qa-3.2 wx_baseu_net-3.2 wx_gtk3u_html-3.2 wx_gtk3u_core-3.2)
  set(lib_dep ${lib_dep} wx_baseu_xml-3.2 wx_baseu-3.2 wx_gtk3u_core-3.2 ${CMAKE_DL_LIBS})
  set(lib_dep ${lib_dep} wxscintilla-3.2 wx_gtk3u_stc-3.2)
  set(lib_dep ${lib_dep} gtk-3 gdk-3 pangocairo-1.0 pango-1.0 harfbuzz atk-1.0 cairo-gobject cairo)
  set(lib_dep ${lib_dep} gdk_pixbuf-2.0 gio-2.0 gobject-2.0 glib-2.0)
  set(lib_dep ${lib_dep} Xtst X11)
  set(lib_dep ${lib_dep} jpeg png z wx_baseu-3.2 expat pcre2-32 c)
endif()

if(APPLE)
  set(lib_dep ${lib_dep} wx_osx_cocoau_xrc-3.2 wx_osx_cocoau_qa-3.2 wx_baseu_net-3.2 wx_osx_cocoau_html-3.2 wx_osx_cocoau_core-3.2)
  set(lib_dep ${lib_dep} wx_baseu_xml-3.2 wx_baseu-3.2 wx_osx_cocoau_webview-3.2)
  set(lib_dep ${lib_dep} wx_osx_cocoau_core-3.2 wxregexu-3.2 wxjpeg-3.2 wxpng-3.2 z iconv)
  set(lib_dep ${lib_dep} wxscintilla-3.2 wx_osx_cocoau_stc-3.2)
  find_library(COREFOUNDATION CoreFoundation)
  if (NOT COREFOUNDATION)
      message(FATAL_ERROR "CoreFoundation not found")
  endif()
  find_library(SECURITY Security)
  find_library(CARBON Carbon)
  find_library(COCOA Cocoa)
  find_library(IOKIT IOKit)
  find_library(WEBKIT WebKit)
  find_library(QUARTZCORE QuartzCore)
  set(lib_dep ${lib_dep} ${COREFOUNDATION} ${SECURITY} ${CARBON} ${COCOA} ${IOKIT} ${WEBKIT} ${QUARTZCORE})
endif()

target_link_libraries(gnostro ${lib_dep})

if (MSVC)
  set_target_properties(gnostro PROPERTIES LINK_FLAGS "/SubSystem:\"Windows\"" VS_DPI_AWARE "ON")
endif()

if(APPLE)
  set(VERSION "1.0")
  set(COPYRIGHT "Sarsat")
  if(NOT IPHONE)
    set_target_properties(gnostro PROPERTIES
      MACOSX_BUNDLE_INFO_PLIST "${CMAKE_SOURCE_DIR}/ext/wxWidgets-3.2.2.1/src/osx/carbon/Info.plist.in"
      RESOURCE "${CMAKE_SOURCE_DIR}/ext/wxWidgets-3.2.2.1/src/osx/carbon/wxmac.icns")
  endif()
  set_target_properties(gnostro PROPERTIES
    MACOSX_BUNDLE_ICON_FILE wxmac.icns
    MACOSX_BUNDLE_LONG_VERSION_STRING "${VERSION}"
    MACOSX_BUNDLE_SHORT_VERSION_STRING "${VERSION}"
    MACOSX_BUNDLE_VERSION "${VERSION}"
    MACOSX_BUNDLE_COPYRIGHT "${COPYRIGHT}"
    MACOSX_BUNDLE_GUI_IDENTIFIER "org.wxwidgets.nostr"
    )
endif()



