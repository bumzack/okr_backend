package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.*;
import at.bumzack.reference.impl.repository.Art2ImgRepository;
import at.bumzack.reference.impl.repository.ArticleRepository;
import at.bumzack.reference.impl.repository.ImageRepository;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Objects;

import static at.bumzack.reference.impl.ImageUtilsV1.convertImage;
import static at.bumzack.reference.impl.ImageUtilsV1.createPPMFile;


@Component
public class ArticleServiceV1 {

    public static final String PROPERTY_CODE = "code";
    private static final Logger LOG = LogManager.getLogger(ArticleServiceV1.class);
    private final ArticleRepository articleRepository;
    private final Art2ImgRepository art2ImgRepository;
    private final ImageRepository imageRepository;
    private final ResolutionService resolutionService;

    private final ObjectMapper mapper;

    public ArticleServiceV1(final ArticleRepository articleRepository,
                            final Art2ImgRepository art2ImgRepository,
                            final ImageRepository imageRepository,
                            final ResolutionService resolutionService,
                            final ObjectMapper mapper) {
        this.articleRepository = articleRepository;
        this.art2ImgRepository = art2ImgRepository;
        this.imageRepository = imageRepository;
        this.resolutionService = resolutionService;
        this.mapper = mapper;
    }


    public List<Article> findPaginated(final int pageNumber, final int pageSize) {
        final var p = PageRequest.of(pageNumber, pageSize, Sort.by(Sort.Direction.ASC, PROPERTY_CODE));
        final var resolutions = resolutionService.findAll();
        final var articles = articleRepository.findAll(p);
        return articles.stream()
                .map(a -> findArticlesAndMapToFullArticle(a, resolutions))
                .toList();
    }

    private Article findArticlesAndMapToFullArticle(final ArticleModel articleModel, final List<Resolution> resolutions) {
        final var imgIds = art2ImgRepository.findByArticleId(articleModel.getId()).stream()
                .map(Art2ImgModel::getImageId)
                .toList();

        final var images = imageRepository.findByIdIn(imgIds).stream()
                .toList();

        final var resizedPngs = images.stream()
                .flatMap(i -> Objects.requireNonNull(resizeImage(i, resolutions)).stream())
                .filter(Objects::nonNull)
                .toList();

        final var fullArticle = new Article();
        fullArticle.setTitle(articleModel.getTitle());
        fullArticle.setDescription(articleModel.getDescription());
        fullArticle.setImages(resizedPngs);
        fullArticle.setCode(articleModel.getCode());
        return fullArticle;
    }

    private List<Image> resizeImage(final ImageModel img, final List<Resolution> resolutions) {
        return resolutions.stream()
                .map(resolution -> {
                    final var pixels = convertToPixelArray(img.getImageJson());
                    final var newPixels = convertImage(pixels, img.getWidth(), img.getHeight(), resolution);

                    try {
                        final StringBuilder ppm = createPPMFile(newPixels, resolution.getWidth(), resolution.getHeight());
                        final var finalImage = new Image();
                        finalImage.setId(img.getId());
                        finalImage.setFilename(img.getFilename());
                        finalImage.setImage(ppm.toString());
                        finalImage.setResolution(resolution.getName());
                        if (resolution.isOriginal()) {
                            finalImage.setWidth(img.getWidth());
                            finalImage.setHeight(img.getHeight());
                        } else {
                            finalImage.setWidth(resolution.getWidth());
                            finalImage.setHeight(resolution.getHeight());
                        }

                        return finalImage;
                    } catch (final Exception e) {
                        LOG.error("error creating the PPM   {}", e.getMessage());
                    }
                    return null;
                })
                .toList();
    }

    private List<Pixel> convertToPixelArray(final String json) {
        try {
            final TypeReference<List<Pixel>> type = new TypeReference<>() {
            };
            return mapper.readValue(json, type);
        } catch (final Exception e) {
            LOG.error("error deserializing the image {}", e.getMessage());
            LOG.error(e);
        }
        return null;
    }
}
