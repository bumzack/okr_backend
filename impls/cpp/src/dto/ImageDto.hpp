#ifndef ImageDto_hpp
#define ImageDto_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)


class ImageDto : public oatpp::DTO {
    DTO_INIT(ImageDto, DTO)

    DTO_FIELD(UInt32, id);

    DTO_FIELD(String, filename, "filename");

    DTO_FIELD(String, image_as_rgb_png, "image_as_rgb_png");

    DTO_FIELD(String, image_as_json_pixels_array, "image_as_json_pixels_array");

    DTO_FIELD(UInt32, width, "width");

    DTO_FIELD(UInt32, height, "height");
};

#include OATPP_CODEGEN_END(DTO)

#endif /* ImageDto_hpp */
