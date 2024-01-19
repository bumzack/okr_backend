
#ifndef EXAMPLE_POSTGRESQL_ART2IMGDB_HPP
#define EXAMPLE_POSTGRESQL_ART2IMGDB_HPP

#include "oatpp-postgresql/orm.hpp"

#include OATPP_CODEGEN_BEGIN(DbClient) //<- Begin Codegen

/**
 * IMAGEDB client definitions.
 */
class Art2ImgDb : public oatpp::orm::DbClient {
public:

    explicit Art2ImgDb(const std::shared_ptr<oatpp::orm::Executor> &executor)
            : oatpp::orm::DbClient(executor) {

        auto version = executor->getSchemaVersion();
        OATPP_LOGD("Art2ImgDb", "constructor - OK. Version=%ld.", version);
    }

    QUERY(getImagesForArticleId,
          "SELECT * FROM art2img WHERE article_id = :id;",
          PREPARE(true), //<-- user prepared statement!
          PARAM(oatpp::UInt32, id))

};

#include OATPP_CODEGEN_END(DbClient) //<- End Codegen

#endif //EXAMPLE_POSTGRESQL_ART2IMGDB_HPP
