//package at.bumzack.reference.impl;
//
//import at.bumzack.reference.impl.dto.*;
//import at.bumzack.reference.impl.repository.Art2ImgRepository;
//import at.bumzack.reference.impl.repository.ArticleRepository;
//import at.bumzack.reference.impl.repository.ArticleRepositoryV3;
//import at.bumzack.reference.impl.repository.ImageRepository;
//import com.fasterxml.jackson.core.type.TypeReference;
//import com.fasterxml.jackson.databind.ObjectMapper;
//import org.apache.logging.log4j.LogManager;
//import org.apache.logging.log4j.Logger;
//import org.springframework.data.domain.PageRequest;
//import org.springframework.data.domain.Sort;
//import org.springframework.stereotype.Component;
//
//import java.util.ArrayList;
//import java.util.List;
//import java.util.Objects;
//import java.util.stream.Collectors;
//
//import static at.bumzack.reference.impl.ImageUtilsV2.convertImageV2;
//
//
//@Component
//public class ArticleServiceV3 {
//
//    private static final String PROPERTY_CODE = "code";
//    private static final Logger LOG = LogManager.getLogger(ArticleServiceV3.class);
//    private final ArticleRepositoryV3 articleRepositoryV3;
//    private final Art2ImgRepository art2ImgRepository;
//    private final ImageRepository imageRepository;
//    private final ResolutionService resolutionService;
//
//    private final ObjectMapper mapper;
//
//    public ArticleServiceV3(final ArticleRepository articleRepository,
//                            final ArticleRepositoryV3 articleRepositoryV3, final Art2ImgRepository art2ImgRepository,
//                            final ImageRepository imageRepository,
//                            final ResolutionService resolutionService,
//                            final ObjectMapper mapper) {
//        this.articleRepositoryV3 = articleRepositoryV3;
//        this.art2ImgRepository = art2ImgRepository;
//        this.imageRepository = imageRepository;
//        this.resolutionService = resolutionService;
//        this.mapper = mapper;
//    }
//
//    private static StringBuilder createPPMFile(final List<Pixel> pixels, final int width, final int height) {
//        // create a PPM file format
//        final StringBuilder ppm = new StringBuilder();
//        ppm.append("P3");
//        ppm.append("\n");
//        final var s = String.format("%d %d", width, height);
//        ppm.append(s);
//        ppm.append("\n");
//        ppm.append("255");
//        ppm.append("\n");
//
//        StringBuilder line = new StringBuilder();
//
//        for (int y = 0; y < height; y++) {
//            for (int x = 0; x < width; x++) {
//                final var idx = y * width + x;
//                final var p = pixels.get(idx);
//
//                final var pixelsAsString = String.format("%d %d %d ", p.getR(), p.getG(), p.getB());
//                if (line.length() + pixelsAsString.length() > 70) {
//                    ppm.append(line);
//                    ppm.append("\n");
//                    line = new StringBuilder();
//                    line.append(pixelsAsString);
//                } else {
//                    line.append(pixelsAsString);
//                }
//            }
//            ppm.append(line);
//            line = new StringBuilder();
//        }
//        return ppm;
//    }
//
//    public List<ArticleAndImageModel> findPaginated(final int pageNumber, final int pageSize) {
//        final var p = PageRequest.of(pageNumber, pageSize, Sort.by(Sort.Direction.ASC, PROPERTY_CODE));
//
//        final var resolutions = resolutionService.findAll();
//
//        final String db = resolutions.stream()
//                .map(Resolution::toString)
//                .collect(Collectors.joining(" // "));
//
//        final var offset = pageNumber * pageSize;
//        final var articles = articleRepositoryV3.getItAll(offset, pageSize);
//        articles.forEach(a -> {
//            LOG.info("article code {},   image id {}, image width {}, image height {}, image  filename {}",
//                    a.getCode(), a.getId(), a.getWidth(), a.getHeight(), a.getFilename());
//        });
////        return articles.stream()
////                .map(a -> findArticlesAndMapToFullArticle(a, resolutions))
////                .toList();
//        return articles;
//    }
//
//    private Article findArticlesAndMapToFullArticle(final ArticleModel articleModel, final List<Resolution> resolutions) {
//        final var imgIds = art2ImgRepository.findByArticleId(articleModel.getId()).stream()
//                .map(Art2ImgModel::getImageId)
//                .toList();
//
//        final var images = imageRepository.findByIdIn(imgIds).stream()
//                .toList();
//
//        final var resizedPngs = images.stream()
//                .flatMap(i -> Objects.requireNonNull(resizeImage(i, resolutions)).stream())
//                .filter(Objects::nonNull)
//                .toList();
//
//        final var fullArticle = new Article();
//        fullArticle.setTitle(articleModel.getTitle());
//        fullArticle.setDescription(articleModel.getDescription());
//        fullArticle.setImages(resizedPngs);
//        fullArticle.setCode(articleModel.getCode());
//        return fullArticle;
//    }
//
//    private List<Image> resizeImage(final ImageModel img, final List<Resolution> resolutions) {
//        final List<Image> res = new ArrayList<>();
//        resolutions.forEach(resolution -> convertImageAndAddToList(img, resolution, res));
//        return res;
//    }
//
//    private void convertImageAndAddToList(final ImageModel img, final Resolution resolution, final List<Image> res) {
//        try {
//            final var json = img.getImageJson();
//            final var pixels = convertToPixelArray(json);
//            if (resolution.isOriginal()) {
//                resolution.setWidth(img.getWidth());
//                resolution.setHeight(img.getHeight());
//            }
//            final var ppm = convertImageV2(pixels, img.getWidth(), img.getHeight(), resolution);
//
//            final var finalImage = new Image();
//            finalImage.setId(img.getId());
//            finalImage.setFilename(img.getFilename());
//            finalImage.setImage(ppm);
//            finalImage.setResolution(resolution.getName());
//            if (resolution.isOriginal()) {
//                finalImage.setWidth(img.getWidth());
//                finalImage.setHeight(img.getHeight());
//            } else {
//                finalImage.setWidth(resolution.getWidth());
//                finalImage.setHeight(resolution.getHeight());
//            }
//            res.add(finalImage);
//        } catch (final Exception e) {
//            LOG.error("error while resizing the image {}", e.getMessage());
//            LOG.error(e.getCause());
//        }
//    }
//
//    private List<Pixel> convertToPixelArray(final String json) {
//        try {
//            final TypeReference<List<Pixel>> type = new TypeReference<>() {
//            };
//            return mapper.readValue(json, type);
//        } catch (final Exception e) {
//            LOG.error("error deserializing the image {}", e.getMessage());
//            LOG.error(e);
//        }
//        return null;
//    }
//}
