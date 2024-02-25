cmake_minimum_required(VERSION 3.11)

project(GFSK)
set(CMAKE_CXX_STANDARD 17)

#set(Boost_USE_STATIC_LIBS ON)
#set(Boost_USE_STATIC_RUNTIME ON)
#add_subdirectory(poco)
#add_subdirectory(cpp-httplib)
#find_package(Threads REQUIRED)
#find_package(Boost 1.79 COMPONENTS system)
add_subdirectory(hackrf)
add_subdirectory(json)
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

target_link_libraries(gfsk hackrf-static ${LIBUSB_LIBRARIES} nlohmann_json::nlohmann_json)
#target_include_directories(gfsk ${Boost_INCLUDE_DIRS})


