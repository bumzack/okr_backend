#ifndef ResolutionDto_hpp
#define ResolutionDto_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)


class ResolutionDto : public oatpp::DTO {

    DTO_INIT(ResolutionDto, DTO)

    DTO_FIELD(UInt32, id);

    DTO_FIELD(String, resolution, "resolution");

};

#include OATPP_CODEGEN_END(DTO)

#endif /* ResolutionDto_hpp */
