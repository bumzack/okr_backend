
#include <iostream>
#include <numeric>
#include <sstream>
#include "ArticleService.hpp"
#include "dto/ResolutionDto.hpp"
#include <numeric>

using namespace std;


oatpp::Object<PageDto<oatpp::Object<ArticleModelDto>>>
ArticleService::getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit) {


    oatpp::UInt32 countToFetch = limit;

    if (limit > 10) {
        countToFetch = 10;
    }

    auto resolutions = getResolutions();

    auto dbResult = m_database->getAllArticles(offset, countToFetch);
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<ArticleModelDto>>>();

    auto iter = items->begin();
    cout << "items iters begin " << endl;
    for (iter; iter < items->end(); iter++) {
        auto image_ids = getImageIds(*iter->get()->id);
        cout << "got " << image_ids->size() << " images for article id: " << *iter->get()->id << " code: "
             << *iter->get()->code << endl;


        // WTF ?
        string ids;
        ids.clear();
        for (auto p = image_ids->begin(); p != image_ids->end(); ++p) {
            auto img_id = p.base()->get()->image_id;
            ids += std::to_string(img_id);
            if (p != image_ids->end() - 1)
                ids += ',';
        }
        std::string delim = ", ";

//         auto ids2 =  std::accumulate(image_ids->begin(), image_ids->end(), std::string(),
//                               [&delim](oatpp::data::mapping::type::DTOWrapper<Art2ImgDto> &x, oatpp::data::mapping::type::DTOWrapper<Art2ImgDto> y) {
//                                   return   std::to_string( x->image_id) + delim + std::to_string( y->image_id);
//                               });

        cout << "ids  joined: " << ids.data() << endl;
    }
    cout << "items iter end " << endl;

    auto page = PageDto<oatpp::Object<ArticleModelDto>>::createShared();
    page->offset = offset;
    page->limit = countToFetch;
    page->count = items->size();
    page->items = items;

    return page;

}

vector<oatpp::data::mapping::type::DTOWrapper<ResolutionDto>> *ArticleService::getResolutions() {
    auto tmp = m_database_resolution->getAllResolutions();
    auto res = tmp->fetch<oatpp::Vector<oatpp::Object<ResolutionDto>>>();

    auto res2 = res.get();

    auto iter = res->begin();
    cout << "iter begin " << endl;
    for (iter; iter < res->end(); iter++) {
        // access value in the memory to which the pointer
        // is referencing
        cout << " id: " << *iter->get()->id << " resolution: " << *iter->get()->resolution << endl;
    }
    cout << "iter end " << endl;

    return res2;
}

vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *
ArticleService::getImageIds(const oatpp::UInt32 &article_id) {
    auto tmp = m_database_art2img->getImagesForArticleId(article_id);
    auto res = tmp->fetch<oatpp::Vector<oatpp::Object<Art2ImgDto>>>();
    cout << "image  query result count items  " << res->size() << endl;

    auto res2 = res.get();

    auto iter = res2->begin();
    cout << "image  iter begin " << endl;
    for (iter; iter < res2->end(); iter++) {
        // access value in the memory to which the pointer
        // is referencing
        cout << "image id: " << *iter->get()->id << " article_id : " << *iter->get()->article_id << " iamge_id : "
             << *iter->get()->image_id << endl;
    }
    cout << "image  iter end " << endl;

    return res2;
}

vector<oatpp::data::mapping::type::DTOWrapper<ImageDto>> *ArticleService::getImages(const oatpp::String &ids) {
    auto tmp = m_database_image->getImagesByIds(ids);
    auto res = tmp->fetch<oatpp::Vector<oatpp::Object<ImageDto>>>();
    cout << "image  query result count items  " << res->size() << endl;

    auto res2 = res.get();

    auto iter = res2->begin();
    cout << "image  iter begin " << endl;
    for (iter; iter < res2->end(); iter++) {
        // access value in the memory to which the pointer
        // is referencing
        cout << "image id: " << *iter->get()->id << " width : " << *iter->get()->width << " height : "
             << *iter->get()->height << endl;
    }
    cout << "image  iter end " << endl;

    return res2;
}

std::string
ArticleService::join(vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *items,
                     std::string delim) {
    cout << "bla " << endl;
    std::string ret = "hall";
    for (const auto &s: *items) {
        if (!ret.empty())
            ret += delim;
        cout << "adding " << s->image_id << " to ids" << endl;
        ret += std::to_string(s->image_id);
    }
    cout << "adding    to ids" << endl;

    return ret;
}

void ArticleService::join2(const vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *items, char delim,
                           string &result) {
    result.clear();
    for (auto p = items->begin(); p != items->end(); ++p) {
        auto img_id = p.base()->get()->image_id;
        result += std::to_string(img_id);
        if (p != items->end() - 1)
            result += delim;
    }
}



