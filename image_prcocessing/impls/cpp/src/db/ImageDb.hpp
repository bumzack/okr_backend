
#ifndef EXAMPLE_POSTGRESQL_IMAGEDB_HPP
#define EXAMPLE_POSTGRESQL_IMAGEDB_HPP

#include "oatpp-postgresql/orm.hpp"

#include OATPP_CODEGEN_BEGIN(DbClient) //<- Begin Codegen

/**
 * IMAGEDB client definitions.
 */
class ImageDb : public oatpp::orm::DbClient {
public:

    explicit ImageDb(const std::shared_ptr<oatpp::orm::Executor> &executor)
            : oatpp::orm::DbClient(executor) {

        auto version = executor->getSchemaVersion();
        OATPP_LOGD("ImageDb", "constructor - OK. Version=%ld.", version);
    }

    QUERY(getImagesByIds,
          "SELECT * FROM images WHERE id IN (:ids);",
          PREPARE(true), //<-- user prepared statement!
          PARAM(oatpp::String, ids))

};

#include OATPP_CODEGEN_END(DbClient) //<- End Codegen

#endif //EXAMPLE_POSTGRESQL_IMAGEDB_HPP
