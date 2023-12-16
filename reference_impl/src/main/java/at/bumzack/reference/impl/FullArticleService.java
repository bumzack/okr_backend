package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Art2Img;
import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.FullArticle;
import at.bumzack.reference.impl.dto.FullImage;
import at.bumzack.reference.impl.dto.MyImage;
import at.bumzack.reference.impl.dto.Resolution;
import at.bumzack.reference.impl.repository.Art2ImgRepository;
import at.bumzack.reference.impl.repository.ArticleRepository;
import at.bumzack.reference.impl.repository.ImageRepository;
import at.bumzack.reference.impl.repository.ResolutionRepository;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.stereotype.Component;
import org.springframework.util.StringUtils;

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
import java.util.Random;
import java.util.stream.Collectors;

import static java.awt.image.BufferedImage.TYPE_3BYTE_BGR;


@Component
public class FullArticleService {

    private static final Logger LOG = LogManager.getLogger(FullArticleService.class);

    private final ArticleRepository articleRepository;
    private final Art2ImgRepository art2ImgRepository;
    private final ImageRepository imageRepository;
    private final ResolutionRepository resolutionRepository;

    public FullArticleService(final ArticleRepository articleRepository,
                              final Art2ImgRepository art2ImgRepository,
                              final ImageRepository imageRepository, ResolutionRepository resolutionRepository) {
        this.articleRepository = articleRepository;
        this.art2ImgRepository = art2ImgRepository;
        this.imageRepository = imageRepository;
        this.resolutionRepository = resolutionRepository;
    }

    public List<FullArticle> findAll() {
        final var resolutions = resolutionRepository.findAll();
        final var articles = articleRepository.findAll();

        return articles.stream()
                .map(a -> findArticlesAndMapToFullArticle(a, resolutions))
                .toList();
    }

    private FullArticle findArticlesAndMapToFullArticle(final Article article, final List<Resolution> resolutions) {
        final var imgIds = art2ImgRepository.findByArticleId(article.getId()).stream()
                .map(Art2Img::getImageId)
                .toList();

        final var images = imageRepository.findByIdIn(imgIds).stream()
                .toList();

        final var resizedPngs = images.stream()
                .flatMap(i -> Objects.requireNonNull(resizeImage(i, resolutions)).stream())
                .filter(Objects::nonNull)
                .toList();

        final var fullArticle = new FullArticle();
        fullArticle.setTitle(article.getTitle());
        fullArticle.setDescription(article.getDescription());
        fullArticle.setImages(resizedPngs);

        return fullArticle;
    }

    private List<FullImage> resizeImage(final MyImage i, final List<Resolution> resolutions) {
        final List<FullImage> res = new ArrayList<>();
        // final var tmp = resolutions.stream().map(r -> r.getResolution()).collect(Collectors.joining(" // "));
       // LOG.info("resolutions   {}", tmp);

        resolutions.forEach(resolution -> {
            try {
                final var widthHeight = StringUtils.split(resolution.getResolution(), "x");
                // LOG.info("widthHeight   {}", widthHeight);
                assert widthHeight != null;
                final var w = widthHeight[0];
                final var h = widthHeight[1];
//                LOG.info("widthHeight[0]   {}", widthHeight[0]);
//                LOG.info("widthHeight[1]   {}", widthHeight[1]);
//
//                LOG.info("w   '{}'", w);
//                LOG.info("h   '{}'", h);

                final int scaledWidth = Integer.parseInt(w);
                final int scaledHeight = Integer.parseInt(h);

//                LOG.info("scaledWidth       {}", scaledWidth);
//                LOG.info("scaledHeight      {}", scaledHeight);

                final Random r = new Random();
                final int i1 = r.nextInt();
                // final var filename = "./file_" + i1 + ".png";

                final byte[] data = Base64.getDecoder().decode(i.getImage());
                // final var bis = new ByteArrayInputStream(data);

                // final BufferedImage bImage = ImageIO.read(bis);
                // ImageIO.write(bImage, "png", new File(filename));

                final var output = resize(data, scaledWidth, scaledHeight);

                final var os = new ByteArrayOutputStream();
                ImageIO.write(output, "png", os);

                final var bytes = os.toByteArray();
                final var im = Base64.getEncoder().encodeToString(bytes);

                final var finalImage = new FullImage();
                finalImage.setCode(i.getCode());
                finalImage.setFilename(i.getFilename());
                finalImage.setImage(im);
                finalImage.setResolution(resolution.getResolution());

                res.add(finalImage);
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
