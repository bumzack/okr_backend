
#ifndef EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP
#define EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP

#include "oatpp-postgresql/orm.hpp"

#include OATPP_CODEGEN_BEGIN(DbClient) //<- Begin Codegen

/**
 * RESOLUTIONDB client definitions.
 */
class RESOLUTIONDB : public oatpp::orm::DbClient {
public:

    explicit RESOLUTIONDB(const std::shared_ptr<oatpp::orm::Executor> &executor)
            : oatpp::orm::DbClient(executor) {

//        oatpp::orm::SchemaMigration migration(executor);
//        migration.addFile(1 /* start from version 1 */, DATABASE_MIGRATIONS "/001_init.sql");
//        // TODO - Add more migrations here.
//        migration.migrate(); // <-- run migrations. This guy will throw on error.
//
       auto version = executor->getSchemaVersion();
//        OATPP_LOGD("RESOLUTIONDB", "Migration - OK. Version=%ld.", version);
        OATPP_LOGD("RESOLUTIONDB", "constructor - OK. Version=%ld.", version);
    }

    QUERY(getAllArticles,
          "SELECT * FROM articles LIMIT :limit OFFSET :offset;",
          PREPARE(true), //<-- user prepared statement!
          PARAM(oatpp::UInt32, offset),
          PARAM(oatpp::UInt32, limit))

};

#include OATPP_CODEGEN_END(DbClient) //<- End Codegen

#endif //EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP
