project(testmy VERSION 1.0.0 DESCRIPTION "test prj")

ADD_LIBRARY( ${PROJECT_NAME} SHARED
	testapi.c
	testapi.h
)
target_include_directories(${PROJECT_NAME} PRIVATE .)
set_target_properties(${PROJECT_NAME} PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(${PROJECT_NAME} PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER testapi.h)
