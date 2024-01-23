//#ifndef EXAMPLE_POSTGRESQL_PAGEDTO_HPP
//#define EXAMPLE_POSTGRESQL_PAGEDTO_HPP
//
//#include "oatpp/core/macro/codegen.hpp"
//#include "oatpp/core/Types.hpp"
//
//#include OATPP_CODEGEN_BEGIN(DTO)
//#include "ArticlelDto.hpp"
//
//template<class T>
//class PageDto : public oatpp::DTO {
//
//    DTO_INIT(PageDto, DTO)
//
//    DTO_FIELD(UInt32, offset);
//    DTO_FIELD(UInt32, limit);
//    DTO_FIELD(UInt32, count);
//    DTO_FIELD(Vector<T>, items);
//
//};
//
//class ArticlePageDto : public PageDto<oatpp::Object<ArticleDto>> {
//    DTO_INIT(ArticlePageDto, PageDto<oatpp::Object<ArticleDto>>)
//};
//
//#include OATPP_CODEGEN_END(DTO)
//
//#endif //EXAMPLE_POSTGRESQL_PAGEDTO_HPP