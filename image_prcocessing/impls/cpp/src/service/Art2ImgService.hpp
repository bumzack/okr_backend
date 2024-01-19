
#ifndef EXAMPLE_POSTGRESQL_ART2IMGSERVICE_HPP
#define EXAMPLE_POSTGRESQL_ART2IMGSERVICE_HPP


#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ResolutionDb.hpp"
#include "dto/ResolutionDto.hpp"
#include "dto/ImageDto.hpp"
#include "db/ImageDb.hpp"
#include "db/Art2ImgDb.hpp"
#include "dto/Art2ImgDto.hpp"

class Art2ImgService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<Art2ImgDb>, m_database); // Inject database component
public:
    std::vector<oatpp::data::mapping::type::DTOWrapper<Art2ImgDto>> *
    getImageIdsForArticleCode(const oatpp::UInt32 &article_id);

};

#endif //EXAMPLE_POSTGRESQL_ART2IMGSERVICE_HPP
