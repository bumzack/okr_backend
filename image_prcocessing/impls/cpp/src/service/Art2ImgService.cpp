
#include <iostream>
#include "ResolutionService.hpp"
#include "dto/ImageDto.hpp"
#include "ImageService.hpp"
#include "dto/Art2ImgDto.hpp"
#include "Art2ImgService.hpp"

using namespace std;

std::vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *
Art2ImgService::getImageIdsForArticleCode(const oatpp::UInt32 &article_id) {

    auto dbResult = m_database->getImagesForArticleId(article_id);
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<Art2ImgDto>>>().get();

    cout << "art2img   found " << items->size() << " images" << endl;
    return items;
}
