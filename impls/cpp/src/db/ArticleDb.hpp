
#ifndef EXAMPLE_POSTGRESQL_ARTICLEDB_HPP
#define EXAMPLE_POSTGRESQL_ARTICLEDB_HPP

#include "oatpp-postgresql/orm.hpp"

#include OATPP_CODEGEN_BEGIN(DbClient) //<- Begin Codegen

/**
 * ArticleDb client definitions.
 */
class ArticleDb : public oatpp::orm::DbClient {
public:

    explicit ArticleDb(const std::shared_ptr<oatpp::orm::Executor> &executor)
            : oatpp::orm::DbClient(executor) {

//        oatpp::orm::SchemaMigration migration(executor);
//        migration.addFile(1 /* start from version 1 */, DATABASE_MIGRATIONS "/001_init.sql");
//        // TODO - Add more migrations here.
//        migration.migrate(); // <-- run migrations. This guy will throw on error.
//
       auto version = executor->getSchemaVersion();
//        OATPP_LOGD("ArticleDb", "Migration - OK. Version=%ld.", version);
        OATPP_LOGD("ArticleDb", "constructor - OK. Version=%ld.", version);
    }

    QUERY(getAllArticles,
          "SELECT * FROM articles LIMIT :limit OFFSET :offset;",
          PREPARE(true), //<-- user prepared statement!
          PARAM(oatpp::UInt32, offset),
          PARAM(oatpp::UInt32, limit))

};

#include OATPP_CODEGEN_END(DbClient) //<- End Codegen

#endif //EXAMPLE_POSTGRESQL_ARTICLEDB_HPP
