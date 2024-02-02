#include "ArticleService.hpp"
#include "dto/Article.hpp"

// TODO - SOME CODE HERE

std::vector<Article> ArticleService::import_articles(bool returnItems) {

    Article * dto  =new Article();
    dto->code = "code";
    dto->title = "title";
    dto->description = "description";
    dto->categories = "categories";
    dto->attributes = "attributes";
    dto->price = 23.2345;
    dto->startDate = "startdate";
    dto->endDate = "endDate";


    std::vector<Article> res;
    res.push_back(*dto);

    return res;
}