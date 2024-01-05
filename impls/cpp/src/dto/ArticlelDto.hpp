#ifndef ArticleDto_hpp
#define ArticleDto_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)


class ArticleDto : public oatpp::DTO {

    DTO_INIT(ArticleDto, DTO)

    DTO_FIELD(UInt32, id);

    DTO_FIELD(String, title, "title");

    DTO_FIELD(String, code, "code");

    DTO_FIELD(String, description, "description");

};

#include OATPP_CODEGEN_END(DTO)

#endif /* ArticleDto_hpp */
