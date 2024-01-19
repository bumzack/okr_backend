
#include <iostream>
#include "ResolutionService.hpp"
#include "dto/ImageDto.hpp"
#include "ImageService.hpp"

using namespace std;

std::vector<oatpp::data::mapping::type::DTOWrapper<ImageDto>> *
ImageService::getImagesByIds(const oatpp::String &ids) {

    auto dbResult = m_database->getImagesByIds(ids);
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<ImageDto>>>().get();

    cout << "found " << items->size() << " images" << endl;
    return items;
}
