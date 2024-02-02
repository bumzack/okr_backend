#ifndef IMPORTRESULTDTO_hpp
#define IMPORTRESULTDTO_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"
#include "dto/Article.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

/**
 *  Data Transfer Object. Object containing fields only.
 *  Used in API for serialization/deserialization and validation
 */
class ImportResultDto : public oatpp::DTO {

    DTO_INIT(ImportResultDto, DTO)

    DTO_FIELD(Int64, linesProcessed);

    DTO_FIELD(Int64, dbRowsWritten);

    DTO_FIELD(List < Article >, items);
};

#include OATPP_CODEGEN_END(DTO)

#endif /* IMPORTRESULTDTO_hpp */
