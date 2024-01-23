
#ifndef EXAMPLE_POSTGRESQL_SYSINFODTO_HPP
#define EXAMPLE_POSTGRESQL_SYSINFODTO_HPP

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

class SysinfoDto : public oatpp::DTO {

    DTO_INIT(SysinfoDto, DTO)

    DTO_FIELD(String, author);

    DTO_FIELD(String, language);

    DTO_FIELD(String, framework);

    DTO_FIELD(String, comment);

    DTO_FIELD(Boolean, multithreaded);

};

#include OATPP_CODEGEN_END(DTO)

#endif //EXAMPLE_POSTGRESQL_SYSINFODTO_HPP
