
#ifndef EXAMPLE_POSTGRESQL_PAGEDTO_HPP
#define EXAMPLE_POSTGRESQL_PAGEDTO_HPP

#include "UserDto.hpp"
#include "ArticleModelDto.hpp"

#include OATPP_CODEGEN_BEGIN(DTO)

template<class T>
class PageDto : public oatpp::DTO {

    DTO_INIT(PageDto, DTO)

    DTO_FIELD(UInt32, offset);

    DTO_FIELD(UInt32, limit);

    DTO_FIELD(UInt32, count);

    DTO_FIELD(Vector < T >, items);

};

class UsersPageDto : public PageDto<oatpp::Object<UserDto>> {

    DTO_INIT(UsersPageDto, PageDto<oatpp::Object<UserDto>>)

};

class ArticlesPageDto : public PageDto<oatpp::Object<ArticleModelDto>> {
    DTO_INIT(ArticlesPageDto, PageDto<oatpp::Object<ArticleModelDto>>)
};

#include OATPP_CODEGEN_END(DTO)

#endif //EXAMPLE_POSTGRESQL_PAGEDTO_HPP
