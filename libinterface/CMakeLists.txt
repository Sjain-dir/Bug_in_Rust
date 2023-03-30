cmake_minimum_required(VERSION 3.0)
project(Libinterface C)
add_library(interface STATIC interface.c)
install(TARGETS interface DESTINATION .)