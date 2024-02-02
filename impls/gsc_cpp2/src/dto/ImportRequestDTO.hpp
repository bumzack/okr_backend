#ifndef IMPORTREQUESTDTO_hpp
#define IMPORTREQUESTDTO_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

/**
 *  Data Transfer Object. Object containing fields only.
 *  Used in API for serialization/deserialization and validation
 */
class ImportRequestDto : public oatpp::DTO {
  
  DTO_INIT(ImportRequestDto, DTO)
  
  DTO_FIELD(Boolean, returnItems);

};

#include OATPP_CODEGEN_END(DTO)

#endif /* IMPORTREQUESTDTO_hpp */
