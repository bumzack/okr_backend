
#ifndef EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP
#define EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP

#include "oatpp-postgresql/orm.hpp"

#include OATPP_CODEGEN_BEGIN(DbClient) //<- Begin Codegen

/**
 * RESOLUTIONDB client definitions.
 */
class ResolutionDb : public oatpp::orm::DbClient {
public:

    explicit ResolutionDb(const std::shared_ptr<oatpp::orm::Executor> &executor)
            : oatpp::orm::DbClient(executor) {

//        oatpp::orm::SchemaMigration migration(executor);
//        migration.addFile(1 /* start from version 1 */, DATABASE_MIGRATIONS "/001_init.sql");
//        // TODO - Add more migrations here.
//        migration.migrate(); // <-- run migrations. This guy will throw on error.
//
        auto version = executor->getSchemaVersion();
        OATPP_LOGD("ResolutionDb", "constructor - OK. Version=%ld.", version);
    }

    QUERY(getAllResolutions,
          "SELECT * FROM resolutions "
    )

};

#include OATPP_CODEGEN_END(DbClient) //<- End Codegen

#endif //EXAMPLE_POSTGRESQL_RESOLUTIONDB_HPP
