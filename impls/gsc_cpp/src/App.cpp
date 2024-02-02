
#include "AppComponent.hpp"
#include "DatabaseComponent.hpp"
#include "ServiceComponent.hpp"
#include "SwaggerComponent.hpp"

#include "oatpp-swagger/Controller.hpp"

#include "oatpp/network/Server.hpp"
#include "controller/ArticleController.hpp"

#include <iostream>

#include <fstream>
#include <string>


void run(const oatpp::base::CommandLineArguments &args) {

    AppComponent appComponent(args);
    ServiceComponent serviceComponent;
    SwaggerComponent swaggerComponent;
    DatabaseComponent databaseComponent;

    /* create ApiControllers and add endpoints to router */

    auto router = serviceComponent.httpRouter.getObject();
    oatpp::web::server::api::Endpoints docEndpoints;

    docEndpoints.append(router->addController(ArticleController::createShared())->getEndpoints());

    router->addController(oatpp::swagger::Controller::createShared(docEndpoints));

    /* create server */

    oatpp::network::Server server(serviceComponent.serverConnectionProvider.getObject(),
                                  serviceComponent.serverConnectionHandler.getObject());

    OATPP_LOGD("Server", "Running on port %s...",
               serviceComponent.serverConnectionProvider.getObject()->getProperty("port").toString()->c_str());

    server.run();

}

int main(int argc, const char *argv[]) {

//    oatpp::base::Environment::init();
//
//    run(oatpp::base::CommandLineArguments(argc, argv));
//
//    oatpp::base::Environment::destroy();




    std::ifstream file("Read.txt");
    std::string str;
    while (std::getline(file, str))
    {
        // Process str
    }


    return 0;
}
