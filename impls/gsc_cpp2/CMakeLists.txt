cmake_minimum_required(VERSION 3.1)

set(project_name my-project) ## rename your project here

project(${project_name})

set(CMAKE_CXX_STANDARD 20)

add_library(${project_name}-lib
        src/AppComponent.hpp
        src/controller/SysinfoController.cpp
        src/controller/SysinfoController.hpp
        src/dto/DTOs.hpp
        src/dto/ArticleDTO.hpp
        src/dto/ImportRequestDTO.hpp
        src/dto/ImportResultDto.hpp
        src/dto/SysinfoDTO.hpp

)

## link libs

find_package(oatpp 1.3.0 REQUIRED)

target_link_libraries(${project_name}-lib
        PUBLIC oatpp::oatpp
        PUBLIC oatpp::oatpp-test
)

target_include_directories(${project_name}-lib PUBLIC src)

## add executables

add_executable(${project_name}-exe
        src/App.cpp
        test/app/MyApiTestClient.hpp
        src/controller/ArticleController.hpp
        src/controller/SysinfoController.hpp
)
target_link_libraries(${project_name}-exe ${project_name}-lib)
add_dependencies(${project_name}-exe ${project_name}-lib)

add_executable(${project_name}-test
        test/tests.cpp
        test/app/TestComponent.hpp
        test/app/MyApiTestClient.hpp
        test/MyControllerTest.cpp
        test/MyControllerTest.hpp
)

target_link_libraries(${project_name}-test ${project_name}-lib)
add_dependencies(${project_name}-test ${project_name}-lib)

set_target_properties(${project_name}-lib ${project_name}-exe ${project_name}-test PROPERTIES
        CXX_STANDARD 20
        CXX_EXTENSIONS OFF
        CXX_STANDARD_REQUIRED ON
)

enable_testing()
add_test(project-tests ${project_name}-test)
