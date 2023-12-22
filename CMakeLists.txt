cmake_minimum_required(VERSION 3.27)
project(arduinoKeyBoardRust)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)

add_library(arduinoKeyBoardRust
        src/Keyboard.cpp
        src/Keyboard.h
        src/Keyboard_da_DK.h
        src/Keyboard_de_DE.h
        src/Keyboard_es_ES.h
        src/Keyboard_fr_FR.h
        src/Keyboard_hu_HU.h
        src/Keyboard_it_IT.h
        src/Keyboard_pt_PT.h
        src/Keyboard_sv_SE.h
        src/KeyboardLayout.h
        src/KeyboardLayout_da_DK.cpp
        src/KeyboardLayout_de_DE.cpp
        src/KeyboardLayout_en_US.cpp
        src/KeyboardLayout_es_ES.cpp
        src/KeyboardLayout_fr_FR.cpp
        src/KeyboardLayout_hu_HU.cpp
        src/KeyboardLayout_it_IT.cpp
        src/KeyboardLayout_pt_PT.cpp
        src/KeyboardLayout_sv_SE.cpp)
