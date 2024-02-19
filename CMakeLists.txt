cmake_minimum_required(VERSION 3.11)

project(GFSK)
set(CMAKE_CXX_STANDARD 17)

add_subdirectory(hackrf_transfer)


if(${MSVC})
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /fp:fast")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic -ffast-math")
endif()


add_executable(gfsk
    gfsk.cpp ao.cpp ao.h StreamingBitSync.cpp StreamingBitSync.h
    StreamingFec.cpp StreamingFec.h StreamingGFSK.cpp StreamingGFSK.h
    Transition.h IQSource.h FileIQSource.cpp FileIQSource.h HackRfIQSource.cpp HackRfIQSource.h)

target_link_libraries(gfsk hackrf)

