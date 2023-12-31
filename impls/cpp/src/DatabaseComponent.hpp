
#ifndef EXAMPLE_POSTGRESQL_DATABASECOMPONENT_HPP
#define EXAMPLE_POSTGRESQL_DATABASECOMPONENT_HPP

#include "db/UserDb.hpp"
#include "dto/ConfigDto.hpp"
#include "db/ArticleDb.hpp"
#include "db/ResolutionDb.hpp"
#include "db/ImageDb.hpp"
#include "db/Art2ImgDb.hpp"
#include <iostream>

class DatabaseComponent {
public:

    /**
     * Create database client
     */
    OATPP_CREATE_COMPONENT(std::shared_ptr<UserDb>, userDb)([] {

        OATPP_COMPONENT(oatpp::Object<ConfigDto>, config); // Get config component

        std::cout << "config.dbConnectionString " << config->dbConnectionString->data() << std::endl;
        std::cout << "config.host " << config->host->data() << std::endl;
        std::cout << "config.port " << config->port << std::endl;
        std::cout << "config.swaggerHost " << config->swaggerHost->data() << std::endl;

        /* Create database-specific ConnectionProvider */
        auto connectionProvider = std::make_shared<oatpp::postgresql::ConnectionProvider>(config->dbConnectionString);

        /* Create database-specific ConnectionPool */
        auto connectionPool = oatpp::postgresql::ConnectionPool::createShared(connectionProvider,
                                                                              10 /* max-connections */,
                                                                              std::chrono::seconds(
                                                                                      5) /* connection TTL */);

        /* Create database-specific Executor */
        auto executor = std::make_shared<oatpp::postgresql::Executor>(connectionPool);

        /* Create MyClient database client */
        return std::make_shared<UserDb>(executor);

    }());




    /**
    * Create database client
    */
    OATPP_CREATE_COMPONENT(std::shared_ptr<ArticleDb>, articleDb)([] {

        OATPP_COMPONENT(oatpp::Object<ConfigDto>, config); // Get config component

        std::cout << "config.dbConnectionString " << config->dbConnectionString->data() << std::endl;
        std::cout << "config.host " << config->host->data() << std::endl;
        std::cout << "config.port " << config->port << std::endl;
        std::cout << "config.swaggerHost " << config->swaggerHost->data() << std::endl;



        /* Create database-specific ConnectionProvider */
        auto connectionProvider = std::make_shared<oatpp::postgresql::ConnectionProvider>(config->dbConnectionString);

        /* Create database-specific ConnectionPool */
        auto connectionPool = oatpp::postgresql::ConnectionPool::createShared(connectionProvider,
                                                                              10 /* max-connections */,
                                                                              std::chrono::seconds(
                                                                                      5) /* connection TTL */);

        /* Create database-specific Executor */
        auto executor = std::make_shared<oatpp::postgresql::Executor>(connectionPool);

        /* Create MyClient database client */
        return std::make_shared<ArticleDb>(executor);

    }());


    /**
    * Create database client
    */
    OATPP_CREATE_COMPONENT(std::shared_ptr<ResolutionDb>, resolutionDb)([] {

        OATPP_COMPONENT(oatpp::Object<ConfigDto>, config); // Get config component

        std::cout << "resolutionDb   config.dbConnectionString " << config->dbConnectionString->data() << std::endl;
        std::cout << "resolutionDb   config.host " << config->host->data() << std::endl;
        std::cout << "resolutionDb   config.port " << config->port << std::endl;
        std::cout << "resolutionDb   config.swaggerHost " << config->swaggerHost->data() << std::endl;

        /* Create database-specific ConnectionProvider */
        auto connectionProvider = std::make_shared<oatpp::postgresql::ConnectionProvider>(config->dbConnectionString);

        /* Create database-specific ConnectionPool */
        auto connectionPool = oatpp::postgresql::ConnectionPool::createShared(connectionProvider,
                                                                              10 /* max-connections */,
                                                                              std::chrono::seconds(
                                                                                      5) /* connection TTL */);

        /* Create database-specific Executor */
        auto executor = std::make_shared<oatpp::postgresql::Executor>(connectionPool);

        /* Create MyClient database client */
        return std::make_shared<ResolutionDb>(executor);

    }());

    OATPP_CREATE_COMPONENT(std::shared_ptr<ImageDb>, imageDb)([] {

        OATPP_COMPONENT(oatpp::Object<ConfigDto>, config); // Get config component

        std::cout << "resolutionDb   config.dbConnectionString " << config->dbConnectionString->data() << std::endl;
        std::cout << "resolutionDb   config.host " << config->host->data() << std::endl;
        std::cout << "resolutionDb   config.port " << config->port << std::endl;
        std::cout << "resolutionDb   config.swaggerHost " << config->swaggerHost->data() << std::endl;

        /* Create database-specific ConnectionProvider */
        auto connectionProvider = std::make_shared<oatpp::postgresql::ConnectionProvider>(config->dbConnectionString);

        /* Create database-specific ConnectionPool */
        auto connectionPool = oatpp::postgresql::ConnectionPool::createShared(connectionProvider,
                                                                              10 /* max-connections */,
                                                                              std::chrono::seconds(
                                                                                      5) /* connection TTL */);

        /* Create database-specific Executor */
        auto executor = std::make_shared<oatpp::postgresql::Executor>(connectionPool);

        /* Create MyClient database client */
        return std::make_shared<ImageDb>(executor);

    }());

    OATPP_CREATE_COMPONENT(std::shared_ptr<Art2ImgDb>, art2imgDb)([] {

        OATPP_COMPONENT(oatpp::Object<ConfigDto>, config); // Get config component

        std::cout << "resolutionDb   config.dbConnectionString " << config->dbConnectionString->data() << std::endl;
        std::cout << "resolutionDb   config.host " << config->host->data() << std::endl;
        std::cout << "resolutionDb   config.port " << config->port << std::endl;
        std::cout << "resolutionDb   config.swaggerHost " << config->swaggerHost->data() << std::endl;

        /* Create database-specific ConnectionProvider */
        auto connectionProvider = std::make_shared<oatpp::postgresql::ConnectionProvider>(config->dbConnectionString);

        /* Create database-specific ConnectionPool */
        auto connectionPool = oatpp::postgresql::ConnectionPool::createShared(connectionProvider,
                                                                              10 /* max-connections */,
                                                                              std::chrono::seconds(
                                                                                      5) /* connection TTL */);

        /* Create database-specific Executor */
        auto executor = std::make_shared<oatpp::postgresql::Executor>(connectionPool);

        /* Create MyClient database client */
        return std::make_shared<Art2ImgDb>(executor);

    }());

};

#endif //EXAMPLE_POSTGRESQL_DATABASECOMPONENT_HPP
