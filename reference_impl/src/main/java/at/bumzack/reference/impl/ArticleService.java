package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Art2ImgModel;
import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ArticleModel;
import at.bumzack.reference.impl.dto.Image;
import at.bumzack.reference.impl.dto.ImageModel;
import at.bumzack.reference.impl.dto.Resolution;
import at.bumzack.reference.impl.repository.Art2ImgRepository;
import at.bumzack.reference.impl.repository.ArticleRepository;
import at.bumzack.reference.impl.repository.ImageRepository;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Component;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.Objects;

import static java.awt.image.BufferedImage.TYPE_3BYTE_BGR;


@Component
public class ArticleService {

    private static final Logger LOG = LogManager.getLogger(ArticleService.class);
    public static final String PROPERTY_CODE = "code";

    private final ArticleRepository articleRepository;
    private final Art2ImgRepository art2ImgRepository;
    private final ImageRepository imageRepository;
    private final ResolutionService resolutionService;

    public ArticleService(final ArticleRepository articleRepository,
                          final Art2ImgRepository art2ImgRepository,
                          final ImageRepository imageRepository,
                          final ResolutionService resolutionService) {
        this.articleRepository = articleRepository;
        this.art2ImgRepository = art2ImgRepository;
        this.imageRepository = imageRepository;
        this.resolutionService = resolutionService;
    }

    public List<Article> findAll() {
        final var resolutions = resolutionService.findAll();
        final var articles = articleRepository.findAllByOrderByCodeAsc();

        return articles.stream()
                .map(a -> findArticlesAndMapToFullArticle(a, resolutions))
                .toList();
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

    private List<Image> resizeImage(final ImageModel i, final List<Resolution> resolutions) {
        final List<Image> res = new ArrayList<>();

        resolutions.forEach(resolution -> {
            try {
                if (resolution.isOriginal()) {
                    final var finalImage = new Image();
                    finalImage.setCode(i.getCode());
                    finalImage.setFilename(i.getFilename());
                    finalImage.setImage(i.getImage());
                    finalImage.setResolution(resolution.getName());
                    res.add(finalImage);
                } else {
                    final byte[] data = Base64.getDecoder().decode(i.getImage());
                    final var output = resize(data, resolution.getWidth(), resolution.getHeight());

                    final var os = new ByteArrayOutputStream();
                    ImageIO.write(output, "png", os);

                    final var bytes = os.toByteArray();
                    final var im = Base64.getEncoder().encodeToString(bytes);

                    final var finalImage = new Image();
                    finalImage.setCode(i.getCode());
                    finalImage.setFilename(i.getFilename());
                    finalImage.setImage(im);
                    finalImage.setResolution(resolution.getName());

                    res.add(finalImage);
                }

            } catch (final Exception e) {
                LOG.error("error while resizing the image {}", e.getMessage());
                LOG.error(e);
            }
        });

        return res;
    }

    public BufferedImage resize(final byte[] data, int scaledWidth, int scaledHeight) throws IOException {
        // reads input image
        final var bis = new ByteArrayInputStream(data);
        final BufferedImage bImage = ImageIO.read(bis);

        // creates output image
        final BufferedImage outputImage = new BufferedImage(scaledWidth, scaledHeight, TYPE_3BYTE_BGR);

        // scales the input image to the output image
        final Graphics2D g2d = outputImage.createGraphics();
        g2d.drawImage(bImage, 0, 0, scaledWidth, scaledHeight, null);
        g2d.dispose();

        return outputImage;
    }

}
