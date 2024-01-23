#ifndef ImportResultDto_hpp
#define ImportResultDto_hpp

#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/Types.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)


class ImportResultDto : public oatpp::DTO {

    DTO_INIT(ImportResultDto, DTO)
    DTO_FIELD(UInt32, lines_processed, "lines_processed");
    DTO_FIELD(UInt32, db_rows_written, "db_rows_written");

};

#include OATPP_CODEGEN_END(DTO)

#endif /* ImportResultDto_hpp */
