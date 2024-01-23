
#ifndef EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
#define EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP

#include "dto/PageDto.hpp"
#include "dto/StatusDto.hpp"

#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ArticleDb.hpp"
#include "dto/ImportResultDto.hpp"
#include "dto/ArticlelDto.hpp"
#include "dto/SysInfo.hpp"

using namespace std::chrono;
using namespace std;

struct Article {
public:
    string code;
    string title;
    string description;
    string attributes;
    string categories;
    string pos;
    double price;
    time_t start_date;
    time_t end_date;
};

const int LEN_CODE = 20;
const int LEN_TITLE = 100;
const int LEN_DESC = 1700;
const int LEN_ATTRIBUTES = 200;
const int LEN_CATEGORIES = 200;
const int LEN_POS = 30;
const int LEN_PRICE = 20;
const int LEN_START_DATE = 25;
const int LEN_END_DATE = 25;

class ArticleService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<ArticleDb>, m_database); // Inject database component

public:
    // oatpp::Object<PageDto<oatpp::Object<ArticleDto>>>
    oatpp::data::mapping::type::VectorObjectWrapper<oatpp::data::mapping::type::DTOWrapper<ArticleDto>, oatpp::data::mapping::type::__class::Vector<oatpp::data::mapping::type::DTOWrapper<ArticleDto>>>
    getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit);

    oatpp::Object<ImportResultDto> importArticles();


    oatpp::Object<SysinfoDto> sysinfo();

    Article convert_to_article(string basicString);

    inline void   rtrim(std::string &s);
};

#endif //EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
