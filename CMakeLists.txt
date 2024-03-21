cmake_minimum_required(VERSION 3.11)

project(GFSK)
SET(CMAKE_CXX_STANDARD 20)
set(
    CMAKE_OSX_ARCHITECTURES arm64
)
include("vcpkg/scripts/buildsystems/vcpkg.cmake")
set(CMAKE_TOOLCHAIN_FILE "vcpkg/scripts/buildsystems/vcpkg.cmake" CACHE STRING "" FORCE)
if (WIN32)
  set(VCPKG_TARGET_TRIPLET "x64-windows")
endif()

find_package(Threads)

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/modules)
#set(Boost_USE_STATIC_LIBS ON)
#set(Boost_USE_STATIC_RUNTIME ON)
#add_subdirectory(poco)
#add_subdirectory(cpp-httplib)
#find_package(Threads REQUIRED)
#find_package(Boost 1.79 COMPONENTS system)
add_subdirectory(hackrf)

find_package(nlohmann_json CONFIG REQUIRED)

if(${MSVC})
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /fp:fast")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic -ffast-math -funsafe-math-optimizations -march=native")
endif()


add_executable(gfsk
    gfsk.cpp ao.cpp ao.h StreamingBitSync.cpp StreamingBitSync.h
    StreamingFec.cpp StreamingFec.h StreamingGFSK.cpp StreamingGFSK.h
    Transition.h IQSource.h FileIQSource.cpp FileIQSource.h 
    HackRfIQSource.cpp HackRfIQSource.h Shifter.cpp Shifter.h
    StreamSplitter.cpp StreamSplitter.h stdio_json.h stdio_json.cpp)

target_link_libraries(gfsk hackrf-static ${LIBUSB_LIBRARIES} nlohmann_json::nlohmann_json pthread)
#target_include_directories(gfsk ${Boost_INCLUDE_DIRS})


