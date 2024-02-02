#ifndef ARTICLE_hpp
#define ARTICLE_hpp


#include <string>

class Article {
public:
    Article() = default;

    std::string code;
    std::string title;
    std::string description;
    std::string attributes;
    std::string categories;
    std::string pos;
    double price{};
    std::string startDate;
    std::string endDate;


};


#endif /* ARTICLE_hpp */
