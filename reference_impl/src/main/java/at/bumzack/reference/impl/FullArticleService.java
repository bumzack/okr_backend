package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Art2Img;
import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.FullArticle;
import at.bumzack.reference.impl.dto.MyImage;
import at.bumzack.reference.impl.repository.Art2ImgRepository;
import at.bumzack.reference.impl.repository.ArticleRepository;
import at.bumzack.reference.impl.repository.ImageRepository;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.stereotype.Component;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.util.Base64;
import java.util.List;
import java.util.Objects;
import java.util.Random;


@Component
public class FullArticleService {

    private static final Logger LOG = LogManager.getLogger(FullArticleService.class);
    public static final int IMAGE_TYPE = 5;

    private final ArticleRepository articleRepository;
    private final Art2ImgRepository art2ImgRepository;
    private final ImageRepository imageRepository;

    public FullArticleService(final ArticleRepository articleRepository,
                              final Art2ImgRepository art2ImgRepository,
                              final ImageRepository imageRepository) {
        this.articleRepository = articleRepository;
        this.art2ImgRepository = art2ImgRepository;
        this.imageRepository = imageRepository;
    }

    public List<FullArticle> findAll() {
        final var articles = articleRepository.findAll();

        return articles.stream()
                .map(this::findArticlesAndMapToFullArticle)
                .toList();
    }

    private FullArticle findArticlesAndMapToFullArticle(final Article a) {
        final var imgIds = art2ImgRepository.findByArticleId(a.getId()).stream()
                .limit(1)
                .map(Art2Img::getImageId)
                .toList();

        final var images = imageRepository.findByIdIn(imgIds);

        final var resizedPngs = images.stream()
                .map(this::resizeImage)
                .filter(Objects::nonNull)
                .toList();

        final var fullArticle = new FullArticle();
        fullArticle.setId(a.getId());
        fullArticle.setTitle(a.getTitle());
        fullArticle.setDescription(a.getDescription());
        fullArticle.setImages(resizedPngs);

        return fullArticle;
    }

    private MyImage resizeImage(final MyImage i) {
        LOG.info("========================================================================");
        try {
            final int scaledWidth = 800;
            final int scaledHeight = 600;

            final byte[] data = Base64.getDecoder().decode(i.getImage());
            final var bis = new ByteArrayInputStream(data);

//            final BufferedImage bImage = ImageIO.read(bis);
//            ImageIO.write(bImage, "png", new File("./file1.png"));
//            final var output = resize(data, scaledWidth, scaledHeight);
//           ImageIO.write(output, "png", new File("./resized.png"));

            final var inputImage = ImageIO.read(bis);
            LOG.info("inputIMage   width: {}, height {},   type  {}", inputImage.getWidth(), inputImage.getHeight(), inputImage.getType());
            final var outputImage = new BufferedImage(scaledWidth, scaledHeight, IMAGE_TYPE);
            LOG.info("outputImage   width: {}, height {},   type  {}", outputImage.getWidth(), outputImage.getHeight(), outputImage.getType());
            final Image scaledImage = inputImage.getScaledInstance(scaledWidth, scaledHeight, Image.SCALE_SMOOTH);
//            LOG.info("scaledImage   width: {}, height {},   type  {}", scaledImage.getWidth(), scaledImage.getHeight(), scaledImage.getType());

            if (outputImage.getGraphics().drawImage(scaledImage, 0, 0, null)) {
                final var bufferedImage = new BufferedImage(scaledImage.getWidth(null), scaledImage.getHeight(null), BufferedImage.TYPE_INT_RGB);
                LOG.info("bufferedImage   width: {}, height {},   type  {}", bufferedImage.getWidth(), bufferedImage.getHeight(), bufferedImage.getType());

                final var byteArrayOutputStream = new ByteArrayOutputStream();

                final Random r = new Random();
                final String filename = "./resized " + r.nextInt() + ".png";

                ImageIO.write(bufferedImage, "png", new File(filename));
                final var bytes = byteArrayOutputStream.toByteArray();

                ImageIO.write(bufferedImage, "png", byteArrayOutputStream);
                LOG.info("bufferedImage  after ImageIO.write   width: {}, height {},   type  {}", bufferedImage.getWidth(), bufferedImage.getHeight(), bufferedImage.getType());

                final var im = Base64.getEncoder().encodeToString(bytes);
                final var finalImage = new MyImage();
                finalImage.setId(i.getId());
                finalImage.setCode(i.getCode());
                finalImage.setFilename(i.getFilename());
                finalImage.setImage(im);
                LOG.info("========================================================================");

                return finalImage;
            }
            LOG.error("cant drawImage   ");
            return null;
        } catch (final Exception e) {
            LOG.error("error while resizing the image {}", e.getMessage());
            LOG.error(e);
        }
        return null;
    }

//    public  BufferedImage resize(final  byte[] data,  int scaledWidth, int scaledHeight) throws IOException {
//        // reads input image
//        final var bis = new ByteArrayInputStream(data);
//        final BufferedImage bImage = ImageIO.read(bis);
//
//        // creates output image
//        final BufferedImage outputImage = new BufferedImage(scaledWidth, scaledHeight, IMAGE_TYPE);
//
//        // scales the input image to the output image
//        final Graphics2D g2d = outputImage.createGraphics();
//        g2d.drawImage(bImage, 0, 0, scaledWidth, scaledHeight, null);
//        g2d.dispose();
//
//        return outputImage;
//    }
}
