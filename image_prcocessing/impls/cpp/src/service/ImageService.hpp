
#ifndef EXAMPLE_POSTGRESQL_RESOLUTIONSERVICE_HPP
#define EXAMPLE_POSTGRESQL_RESOLUTIONSERVICE_HPP


#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ResolutionDb.hpp"
#include "dto/ResolutionDto.hpp"
#include "dto/ImageDto.hpp"
#include "db/ImageDb.hpp"

class ImageService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<ImageDb>, m_database); // Inject database component
public:
    std::vector<oatpp::data::mapping::type::DTOWrapper<ImageDto>> *getImagesByIds(const oatpp::String &ids);

};

#endif //EXAMPLE_POSTGRESQL_RESOLUTIONSERVICE_HPP
