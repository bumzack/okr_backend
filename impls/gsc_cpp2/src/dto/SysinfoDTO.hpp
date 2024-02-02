#ifndef SYSINFODTO_hpp
#define SYSINFODTO_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

/**
 *  Data Transfer Object. Object containing fields only.
 *  Used in API for serialization/deserialization and validation
 */
class SysinfoDto : public oatpp::DTO {

    DTO_INIT(SysinfoDto, DTO)

    DTO_FIELD(String, author);

    DTO_FIELD(String, language);

    DTO_FIELD(String, framework);

    DTO_FIELD(Boolean, multithreaded);

    DTO_FIELD(String, comment);

    DTO_FIELD(String, version);

};

#include OATPP_CODEGEN_END(DTO)

#endif /* SYSINFODTO_hpp */
