
#ifndef EXAMPLE_POSTGRESQL_IMAGESERVICE_HPP
#define EXAMPLE_POSTGRESQL_IMAGESERVICE_HPP


#include "oatpp/web/protocol/http/Http.hpp"
#include "oatpp/core/macro/component.hpp"
#include "db/ResolutionDb.hpp"
#include "dto/ResolutionDto.hpp"
#include "db/ImageDb.hpp"

class ResolutionService {
private:
    typedef oatpp::web::protocol::http::Status Status;
private:
    OATPP_COMPONENT(std::shared_ptr<ResolutionDb>, m_database); // Inject database component
public:
    std::vector<oatpp::data::mapping::type::DTOWrapper<ResolutionDto>> *getAllResolutions();

};

#endif //EXAMPLE_POSTGRESQL_IMAGESERVICE_HPP
