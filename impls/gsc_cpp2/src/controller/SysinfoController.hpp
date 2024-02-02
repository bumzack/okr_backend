#ifndef MyController_hpp
#define MyController_hpp


#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/macro/component.hpp"

#include OATPP_CODEGEN_BEGIN(ApiController) //<-- Begin Codegen
#include "dto/SysinfoDTO.hpp"

/**
 * Sample Api Controller.
 */
class SysinfoController : public oatpp::web::server::api::ApiController {
public:
    /**
     * Constructor with object mapper.
     * @param objectMapper - default object mapper used to serialize/deserialize DTOs.
     */
    SysinfoController(OATPP_COMPONENT(std::shared_ptr<ObjectMapper>, objectMapper))
            : oatpp::web::server::api::ApiController(objectMapper) {}

public:

    ENDPOINT("GET", "/api/v1/sysinfo", root) {
        auto dto = SysinfoDto::createShared();
        dto->version = "v1";
        dto->comment = "cpp - lolz!!!";
        dto->author = "gsc";
        dto->framework= "oatpp";
        dto->multithreaded =false;
        dto->language = "C++ 20";

        return createDtoResponse(Status::CODE_200, dto);
    }

};

#include OATPP_CODEGEN_END(ApiController) //<-- End Codegen

#endif /* MyController_hpp */
