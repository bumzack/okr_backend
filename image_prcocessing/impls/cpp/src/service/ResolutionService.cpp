
#include "ResolutionService.hpp"


std::vector<oatpp::data::mapping::type::DTOWrapper<ResolutionDto>> *
ResolutionService::getAllResolutions() {

    auto dbResult = m_database->getAllResolutions();
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<ResolutionDto>>>().get();

    return items;

}
