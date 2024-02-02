#ifndef ArticleService_hpp
#define ArticleService_hpp

#include "dto/ArticleDTO.hpp"
#include "dto/Article.hpp"


/**
 * Sample Api Controller.
 */
class ArticleService {
public:
    /**
     * Constructor with object mapper.
     * @param objectMapper - default object mapper used to serialize/deserialize DTOs.
     */
    ArticleService() = default;

public:
    std::vector<Article> import_articles(bool returnItems);

};


#endif /* ArticleService_hpp */
