
#ifndef UserController_hpp
#define UserController_hpp

#include "service/UserService.hpp"

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/parser/json/mapping/ObjectMapper.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include <ostream>
#include <iostream>

using namespace std;

#include OATPP_CODEGEN_BEGIN(ApiController) //<- Begin Codegen

/**
 * User REST controller.
 */
class UserController : public oatpp::web::server::api::ApiController {
public:
    UserController(const std::shared_ptr<ObjectMapper> &objectMapper)
            : oatpp::web::server::api::ApiController(objectMapper) {}

private:
    UserService m_userService; // Create user service.
public:

    static std::shared_ptr<UserController> createShared(
            OATPP_COMPONENT(std::shared_ptr<ObjectMapper>,
                            objectMapper) // Inject objectMapper component here as default parameter
    ) {
        return std::make_shared<UserController>(objectMapper);
    }

    ENDPOINT_INFO(createUser) {
        info->summary = "Create new User";

        info->addConsumes < Object < UserDto >> ("application/json");

        info->addResponse < Object < UserDto >> (Status::CODE_200, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");
    }

    ENDPOINT("POST", "users", createUser,
             BODY_DTO(Object < UserDto > , userDto)) {
        auto pw = userDto->password;

        cout << "POST new user  user.id    " << userDto->id->data() << endl;
        cout << "POST new user   user.email    " << userDto->email->data() << endl;
        cout << "POST new user   user.password    " << userDto->password->data() << endl;
        cout << "POST new user   user.role    " << userDto->role.get() << endl;
        cout << "POST new user   user.userName    " << userDto->userName.get() << endl;
        return createDtoResponse(Status::CODE_200, m_userService.createUser(userDto));
    }


    ENDPOINT_INFO(putUser) {
        info->summary = "Update User by userId";

        info->addConsumes < Object < UserDto >> ("application/json");

        info->addResponse < Object < UserDto >> (Status::CODE_200, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_404, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");

        info->pathParams["userId"].description = "User Identifier";
    }

    ENDPOINT("PUT", "users/{userId}", putUser,
             PATH(String, userId),
             BODY_DTO(Object < UserDto > , userDto)) {
        userDto->id = userId;
        cout << "PUT  user by ID   " << userId->data() << endl;
        cout << "PUT  user.id    " << userDto->id->data() << endl;
        cout << "PUT  user.email    " << userDto->email->data() << endl;
        cout << "PUT  user.password    " << userDto->password->data() << endl;
        cout << "PUT  user.role    " << userDto->role.get() << endl;
        cout << "PUT  user.userName    " << userDto->userName.get() << endl;

        return createDtoResponse(Status::CODE_200, m_userService.updateUser(userDto));
    }


    ENDPOINT_INFO(getUserById) {
        info->summary = "Get one User by userId";

        info->addResponse < Object < UserDto >> (Status::CODE_200, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_404, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");

        info->pathParams["userId"].description = "User Identifier";
    }

    ENDPOINT("GET", "users/{userId}", getUserById,
             PATH(String, userId)) {
        cout << "get users by id " << userId->data() << endl;

        return createDtoResponse(Status::CODE_200, m_userService.getUserById(userId));
    }


    ENDPOINT_INFO(getUsers) {
        info->summary = "get all stored users";

        info->addResponse<oatpp::Object<UsersPageDto>>(Status::CODE_200, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");
    }

    ENDPOINT("GET", "users/offset/{offset}/limit/{limit}", getUsers,
             PATH(UInt32, offset),
             PATH(UInt32, limit)) {
        cout << "get all users by limit " << limit << "  offset " << offset << endl;
        return createDtoResponse(Status::CODE_200, m_userService.getAllUsers(offset, limit));
    }


    ENDPOINT_INFO(deleteUser) {
        info->summary = "Delete User by userId";

        info->addResponse < Object < StatusDto >> (Status::CODE_200, "application/json");
        info->addResponse < Object < StatusDto >> (Status::CODE_500, "application/json");

        info->pathParams["userId"].description = "User Identifier";
    }

    ENDPOINT("DELETE", "users/{userId}", deleteUser,
             PATH(String, userId)) {
        cout << "delete user by ID   " << userId->data() << endl;
        return createDtoResponse(Status::CODE_200, m_userService.deleteUserById(userId));
    }

};

#include OATPP_CODEGEN_BEGIN(ApiController) //<- End Codegen

#endif /* UserController_hpp */
