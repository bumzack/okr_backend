
#ifndef EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
#define EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP

#include "db/UserDb.hpp"
#include "dto/PageDto.hpp"
#include "dto/StatusDto.hpp"

#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ArticleDb.hpp"

class ArticleService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<ArticleDb>, m_database); // Inject database component
public:
    oatpp::Object<PageDto<oatpp::Object<ArticleDto>>>
    getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit);

};

#endif //EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
