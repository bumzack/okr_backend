
#ifndef ArticleController_hpp
#define ArticleController_hpp

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/parser/json/mapping/ObjectMapper.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include <ostream>
#include <iostream>

using namespace std;

#include "service/ArticleService.hpp"

#include OATPP_CODEGEN_BEGIN(ApiController) //<- Begin Codegen

/**
 * User REST controller.
 */
class ArticleController : public oatpp::web::server::api::ApiController {
public:
    explicit ArticleController(const std::shared_ptr<ObjectMapper> &objectMapper)
            : oatpp::web::server::api::ApiController(objectMapper) {}

private:
    ArticleService m_articleService; // Create user service.
public:

    static std::shared_ptr<ArticleController> createShared(
            OATPP_COMPONENT(std::shared_ptr<ObjectMapper>,
                            objectMapper) // Inject objectMapper component here as default parameter
    ) {
        return std::make_shared<ArticleController>(objectMapper);
    }

    ENDPOINT("GET", "articles/offset/{offset}/limit/{limit}", getArticles,
             PATH(UInt32, offset),
             PATH(UInt32, limit)) {
        cout << "get all articles by limit " << limit << "  offset " << offset << endl;
        return createDtoResponse(Status::CODE_200, m_articleService.getAllArticles(offset, limit));
    }

    ENDPOINT("POST", "articles/import", importArticles) {
        cout << "import articles " << endl;
        return createDtoResponse(Status::CODE_200, m_articleService.importArticles());
    }

    ENDPOINT("GET", "sysinfo", sysinfo) {
        cout << "import articles " << endl;
        return createDtoResponse(Status::CODE_200, m_articleService.sysinfo());
    }

};

#include OATPP_CODEGEN_BEGIN(ApiController) //<- End Codegen

#endif /* ArticleController_hpp */
