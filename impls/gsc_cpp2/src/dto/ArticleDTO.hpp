#ifndef ARTICLEDTO_hpp
#define ARTICLEDTO_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

/**
 *  Data Transfer Object. Object containing fields only.
 *  Used in API for serialization/deserialization and validation
 */
class ArticleDto : public oatpp::DTO {

    DTO_INIT(ArticleDto, DTO)

    DTO_FIELD(String, code);

    DTO_FIELD(String, title);

    DTO_FIELD(String, description);

    DTO_FIELD(String, attributes);

    DTO_FIELD(String, categories);

    DTO_FIELD(String, pos);

    DTO_FIELD(Float64, price);

    DTO_FIELD(String, startDate);

    DTO_FIELD(String, endDate);

};

#include OATPP_CODEGEN_END(DTO)

#endif /* ARTICLEDTO_hpp */
