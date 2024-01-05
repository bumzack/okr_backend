
#ifndef EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
#define EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP

#include "db/UserDb.hpp"
#include "dto/PageDto.hpp"
#include "dto/StatusDto.hpp"

#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ArticleDb.hpp"
#include "db/ResolutionDb.hpp"
#include "db/ImageDb.hpp"
#include "dto/ResolutionDto.hpp"
#include "dto/ImageDto.hpp"
#include "dto/Art2ImgDto.hpp"
#include "db/Art2ImgDb.hpp"

using namespace std;

class ArticleService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<ArticleDb>, m_database); // Inject database component
    OATPP_COMPONENT(std::shared_ptr<ResolutionDb>, m_database_resolution); // Inject database component
    OATPP_COMPONENT(std::shared_ptr<ImageDb>, m_database_image); // Inject database component
    OATPP_COMPONENT(std::shared_ptr<Art2ImgDb>, m_database_art2img); // Inject database component

public:
    oatpp::Object<PageDto<oatpp::Object<ArticleModelDto>>>
    getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit);

    vector<oatpp::data::mapping::type::DTOWrapper<ResolutionDto>> *getResolutions();

    vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *getImageIds(const oatpp::UInt32 &article_id);

    vector<oatpp::data::mapping::type::DTOWrapper<ImageDto>> *getImages(const oatpp::String &ids);

    static std::string join(vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *items, std::string delim);

    static void
    join2(const vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *items, char delim, string &result);
};

#endif //EXAMPLE_POSTGRESQL_ARTICLESERVICE_HPP
