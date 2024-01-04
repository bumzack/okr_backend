#ifndef Art2ImgDto_hpp
#define Art2ImgDto_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)


class Art2ImgDto : public oatpp::DTO {
    DTO_INIT(Art2ImgDto, DTO)
    DTO_FIELD(String, id);
    DTO_FIELD(String, article_id, "article_id");
    DTO_FIELD(String, image_id, "image_id");
};

#include OATPP_CODEGEN_END(DTO)

#endif /* Art2ImgDto_hpp */
