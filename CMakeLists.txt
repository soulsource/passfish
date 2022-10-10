project(harbour-passfish)
cmake_minimum_required(VERSION 3.5)
cmake_policy(SET CMP0046 NEW)
cmake_policy(SET CMP0071 NEW)

LIST(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

find_package (Qt5 COMPONENTS Core Network Qml Gui Quick REQUIRED)
find_package(RustQtBindingGenerator REQUIRED)
find_package(Cargo REQUIRED)
find_package(Rust REQUIRED)

set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
find_package(Threads REQUIRED)

include(FindPkgConfig)
pkg_search_module(SAILFISH sailfishapp REQUIRED)

add_definitions(${SAILFISH_CFLAGS})
link_libraries(${SAILFISH_LDFLAGS})

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

if(NOT DEFINED RUST_TARGET_TRIPLET)
    message( FATAL_ERROR "Auto-Detection of Rust target triplet is currently not implemented. Please set RUST_TARGET_TRIPLET on the cmake command line." )
endif()
message(STATUS "Rust Target=${RUST_TARGET_TRIPLET}")

string(TOUPPER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_UPPER)
if(CMAKE_BUILD_TYPE_UPPER STREQUAL DEBUG)
    set(RUST_TARGET_DIR target/${RUST_TARGET_TRIPLET}/debug/)
    set(RUST_BUILD_FLAG --target=${RUST_TARGET_TRIPLET})
else()
    set(RUST_TARGET_DIR target/${RUST_TARGET_TRIPLET}/release/)
    set(RUST_BUILD_FLAG --release --target=${RUST_TARGET_TRIPLET})
endif()
SET(RUST_DIR "${CMAKE_CURRENT_SOURCE_DIR}/rust")
SET(RUST_LIB "${RUST_DIR}/${RUST_TARGET_DIR}/libpassfish.so")

#Must set RPATH to custom folder where Sailfish wants us to put our shared lib
set(CMAKE_INSTALL_RPATH "/usr/share/harbour-passfish/lib")
set(CMAKE_SKIP_RPATH false)

# generate c++ and rust code from bindings.json
add_custom_command(
  OUTPUT "${RUST_DIR}/src/interface.rs"
         "${CMAKE_CURRENT_SOURCE_DIR}/src/Bindings.h"
         "${CMAKE_CURRENT_SOURCE_DIR}/src/Bindings.cpp"
  COMMAND "${RustQtBindingGenerator_EXECUTABLE}" #--overwrite-implementation
            "${CMAKE_CURRENT_SOURCE_DIR}/bindings.json"
  DEPENDS bindings.json
)

# compile the rust code into a static library
FILE(GLOB_RECURSE RustSources "rust/src/*.rs")
FILE(GLOB_RECURSE RustMacroSources "rust_macro/*.rs")
FILE(GLOB_RECURSE RustTesthelperSources "rust_testhelper/*.rs")
add_custom_command(
  OUTPUT "${RUST_LIB}"
  COMMAND ${Cargo_EXECUTABLE} build ${RUST_BUILD_FLAG}
  DEPENDS rust/Cargo.toml
          ${RustSources}
          rust/src/interface.rs
          rust_macro/Cargo.toml
          ${RustMacroSources}
          rust_testhelper/Cargo.toml
          ${RustTesthelperSources}
  WORKING_DIRECTORY "${RUST_DIR}"
)
add_custom_target(rust_target DEPENDS "${RUST_LIB}")

add_custom_command(
    OUTPUT translations/harbour-passfish.qm translations/harbour-passfish-de.qm
    COMMAND lrelease translations/harbour-passfish.ts translations/harbour-passfish-de.ts
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)
add_custom_target(translations_target DEPENDS translations/harbour-passfish.qm translations/harbour-passfish-de.qm)

add_executable(
    harbour-passfish
    src/PassFish.cpp
    src/Bindings.cpp
    src/pwm_qhash.cpp
    src/GraphemeCountValidator.cpp
)
add_dependencies(harbour-passfish rust_target translations_target)
target_compile_definitions(harbour-passfish PRIVATE
    $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>
)
target_include_directories(harbour-passfish PRIVATE
    $<BUILD_INTERFACE:
    ${SAILFISH_INCLUDE_DIRS}
>)
target_link_libraries(harbour-passfish
    Qt5::Quick
    ${RUST_LIB}
    ${CMAKE_THREAD_LIBS_INIT}
    ${CMAKE_DL_LIBS}
    ${SAILFISH_LDFLAGS}
)

install(TARGETS harbour-passfish
    RUNTIME DESTINATION bin
)
install(FILES ${RUST_LIB}
    DESTINATION share/harbour-passfish/lib
)
install(DIRECTORY qml
    DESTINATION share/harbour-passfish
)
install(DIRECTORY translations
    DESTINATION share/harbour-passfish
    FILES_MATCHING PATTERN "*.qm"
)
install(FILES harbour-passfish.desktop
    DESTINATION share/applications
)
install(FILES icons/86x86/harbour-passfish.png
    DESTINATION share/icons/hicolor/86x86/apps
)
install(FILES icons/108x108/harbour-passfish.png
    DESTINATION share/icons/hicolor/108x108/apps
)
install(FILES icons/128x128/harbour-passfish.png
    DESTINATION share/icons/hicolor/128x128/apps
)
install(FILES icons/172x172/harbour-passfish.png
    DESTINATION share/icons/hicolor/172x172/apps
)

# Get the other files reachable from the project tree in Qt Creator
FILE(GLOB TsFiles "translations/*.ts")
add_custom_target(distfiles
    SOURCES
        harbour-passfish.desktop
        qml/PassFish.qml
        qml/cover/CoverPage.qml
        qml/pages/MainPage.qml
        qml/pages/AboutPage.qml
        qml/pages/ProfilesPage.qml
        qml/pages/ProfileEditor.qml
        qml/pages/SettingsEditor.qml
        qml/components/NoticeOptional.qml
        qml/helpers/NoticeLoadable.qml
        qml/helpers/NotificationLoadable.qml
        rpm/harbour-passfish.changes.in
        rpm/harbour-passfish.changes.run.in
        rpm/harbour-passfish.spec
        rpm/harbour-passfish.yaml
        ${TsFiles})
FILE(GLOB AddCMakeFiles "cmake/*.cmake")
add_custom_target(cmake_helpers
    SOURCES
        PreLoad.cmake
        ${AddCMakeFiles})

add_custom_target(rust_bindings
    SOURCES
        bindings.json
        rust/Cargo.toml
        ${RustSources}
        rust_macro/Cargo.toml
        ${RustMacroSources}
        rust_testhelper/Cargo.toml
        ${RustTesthelperSources})

# Tell Qt Creator where the application executable(s) would be located on the
# device.
#
# It is not necessary to list other deployables than executables (runtime
# targets) here. The deployment process of Sailfish OS projects is opaque to
# Qt Creator and the information contained in QtCreatorDeployment.txt is only
# used to locate the executable associated with the active run configuration
# on the device in order to run it.
#
# Search the Qt Creator Manual to learn about the QtCreatorDeployment.txt file
# format.
file(WRITE "${CMAKE_BINARY_DIR}/QtCreatorDeployment.txt"
    "${CMAKE_INSTALL_PREFIX}\n${CMAKE_BINARY_DIR}/harbour-passfish:bin\n")
