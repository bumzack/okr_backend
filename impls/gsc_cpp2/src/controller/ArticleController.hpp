#ifndef ArticleController_hpp
#define ArticleController_hpp

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/macro/component.hpp"

#include OATPP_CODEGEN_BEGIN(ApiController) //<-- Begin Codegen
#include "dto/SysinfoDTO.hpp"
#include "dto/ArticleDTO.hpp"
#include "dto/ImportResultDto.hpp"
#include "dto/ImportRequestDTO.hpp"
#include "service/ArticleService.hpp"

/**
 * Sample Api Controller.
 */
class ArticleController : public oatpp::web::server::api::ApiController {
public:
    /**
     * Constructor with object mapper.
     * @param objectMapper - default object mapper used to serialize/deserialize DTOs.
     */
    ArticleController(OATPP_COMPONENT(std::shared_ptr<ObjectMapper>, objectMapper))
            : oatpp::web::server::api::ApiController(objectMapper) {}

public:

    ENDPOINT("POST", "/api/v1/articles/import", root,
             BODY_DTO(Object < ImportRequestDto > , import_request)) {

        ArticleService service;
        auto art = service.import_articles(import_request->returnItems);

        auto dto = ImportResultDto::createShared();
        dto->linesProcessed = 1;
        dto->dbRowsWritten = 1;
        dto->items = {};

        for (auto &v: art) {
            // dto->articles->push_back(v);
        }

        return createDtoResponse(Status::CODE_200, dto);
    }

};

#include OATPP_CODEGEN_END(ApiController) //<-- End Codegen

#endif /* ArticleController_hpp */
