
#include "ArticleService.hpp"


oatpp::Object<PageDto<oatpp::Object<ArticleDto>>>
ArticleService::getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit) {

    oatpp::UInt32 countToFetch = limit;

    if (limit > 10) {
        countToFetch = 10;
    }

    auto dbResult = m_database->getAllArticles(offset, countToFetch);
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<ArticleDto>>>();

    auto page = PageDto<oatpp::Object<ArticleDto>>::createShared();
    page->offset = offset;
    page->limit = countToFetch;
    page->count = items->size();
    page->items = items;

    return page;

}
