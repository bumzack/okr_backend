package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Image;
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

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.BufferedWriter;
import java.io.ByteArrayInputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

import static java.awt.image.BufferedImage.TYPE_3BYTE_BGR;


@Component
public class ArticleService {

    public static final String PROPERTY_CODE = "code";
    private static final Logger LOG = LogManager.getLogger(ArticleService.class);
    private final ArticleRepository articleRepository;
    private final Art2ImgRepository art2ImgRepository;
    private final ImageRepository imageRepository;
    private final ResolutionService resolutionService;

    private final ObjectMapper mapper;

    public ArticleService(final ArticleRepository articleRepository,
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

    private static StringBuilder createPPMFile(final List<Pixel> pixels, final int width, final int height) {
        // create a PPM file format
        final StringBuilder ppm = new StringBuilder();
        ppm.append("P3");
        ppm.append("\n");
        final var s = String.format("%d %d", width, height);
        ppm.append(s);
        ppm.append("\n");
        ppm.append("255");
        ppm.append("\n");

        StringBuilder line = new StringBuilder();

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                final var idx = y * width + x;
                final var p = pixels.get(idx);

                final var pixelsAsString = String.format("%d %d %d ", p.getR(), p.getG(), p.getB());
                if (line.length() + pixelsAsString.length() > 70) {
                    ppm.append(line);
                    ppm.append("\n");
                    line = new StringBuilder();
                    line.append(pixelsAsString);
                } else {
                    line.append(pixelsAsString);
                }
            }
            ppm.append(line);
            line = new StringBuilder();
        }
        return ppm;
    }

    public List<Article> findPaginated(final int pageNumber, final int pageSize) {
        final var p = PageRequest.of(pageNumber, pageSize, Sort.by(Sort.Direction.ASC, PROPERTY_CODE));

        final var resolutions = resolutionService.findAll();

        final String db = resolutions.stream()
                .map(Resolution::toString)
                .collect(Collectors.joining(" // "));
        // LOG.info("all resolutions in DB     {}", db);

        final var articles = articleRepository.findAll(p);
        // LOG.info("articles {}", articles.getTotalElements());
        return articles.stream()
                .map(a -> findArticlesAndMapToFullArticle(a, resolutions))
                .toList();
    }

    private Article findArticlesAndMapToFullArticle(final ArticleModel articleModel, final List<Resolution> resolutions) {
        final var imgIds = art2ImgRepository.findByArticleId(articleModel.getId()).stream()
                .map(Art2ImgModel::getImageId)
                .toList();
        //LOG.info("imgIds {}", imgIds.size());

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
        final List<Image> res = new ArrayList<>();
        resolutions.forEach(resolution -> convertImageAndAddToList(img, resolution, res));
        return res;
    }

    private void convertImageAndAddToList(final ImageModel img, final Resolution resolution, final List<Image> res) {
        //LOG.info("resizing images");
        try {
            LOG.info("resolution  {}", resolution);
            final var json = img.getImageJson();
            final var pixels = convertToPixelArray(json);
            //LOG.info("converting to JSON ok ");
            final var ppm = toPPM(pixels, img.getWidth(), img.getHeight(), resolution, img.getFilename());
            //LOG.info("converting to PPM ok ");

//            final var dbImg = createPPMFile(pixels, img.getWidth(), img.getHeight());
//
//            final var filename = String.format("%s_java_orig_db_%s.ppm", img.getFilename(), resolution.getName());
//            final BufferedWriter writer = new BufferedWriter(new FileWriter(filename));
//            writer.write(dbImg.toString());
//            writer.close();
//            LOG.info("wrote PPM to filename {} ", filename);


            final var finalImage = new Image();
            finalImage.setId(img.getId());
            finalImage.setFilename(img.getFilename());
            finalImage.setImage(ppm);
            finalImage.setResolution(resolution.getName());
            if (resolution.isOriginal()) {
                finalImage.setWidth(img.getWidth());
                finalImage.setHeight(img.getHeight());
            } else {
                finalImage.setWidth(resolution.getWidth());
                finalImage.setHeight(resolution.getHeight());
            }
            res.add(finalImage);
        } catch (final Exception e) {
            LOG.error("error while resizing the image {}", e.getMessage());
            LOG.error(e.getCause());
        }
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

    private String toPPM(final List<Pixel> source,
                         final Integer sourceWidth,
                         final Integer sourceHeight,
                         final Resolution resolution,
                         final String filename) throws IOException {

        final List<Pixel> mirroredPix = new ArrayList<>();

//        LOG.info("sourceWidth   {} ,  sourceHeight   {},      resolution.width {}, resolution.height  {}",
//                sourceWidth, sourceHeight, resolution.getWidth(), resolution.getHeight());
        try {
            // mirror image
            for (int yTarget = 0; yTarget < sourceHeight; yTarget++) {
                for (int xTarget = 0; xTarget < sourceWidth; xTarget++) {
                    final var xSource = sourceWidth - xTarget - 1;
                    final var ySource = sourceHeight - yTarget - 1;
                    final var idxSource = ySource * sourceWidth + xSource;
                    final var p = source.get(idxSource);
                    mirroredPix.add(p);
                }
            }
        } catch (final Exception e) {
            LOG.error("error mirroring the image   {}", e.getMessage());
        }

//        final var mirroredPpm = createPPMFile(mirroredPix, sourceWidth, sourceHeight);
//        final var fn = String.format("%s_orig_db_mirrored_%s.ppm", filename, resolution.getName());
//        final BufferedWriter writer = new BufferedWriter(new FileWriter(fn));
//        writer.write(mirroredPpm.toString());
//        writer.close();

//        final var targetSize = resolution.getWidth() * resolution.getHeight();
//        LOG.info("sourceWidth   {} ,  sourceHeight   {},      resolution.width {}, resolution.height  {}   mirrored.length  {}    targetSize   {}",
//                sourceWidth, sourceHeight, resolution.getWidth(), resolution.getHeight(), mirroredPix.size(), targetSize);


        // crop to resolution image
        final List<Pixel> croppedPix = new ArrayList<>();

        try {
            for (int yTarget = 0; yTarget < resolution.getHeight(); yTarget++) {
                for (int xTarget = 0; xTarget < resolution.getWidth(); xTarget++) {
                    final var idxSource = yTarget * sourceWidth + xTarget;
                    final var p = mirroredPix.get(idxSource);
                    croppedPix.add(p);
                }
            }
        } catch (final Exception e) {
            LOG.error("error cropping the image   {}", e.getMessage());
//            LOG.info("sourceWidth   {} ,  sourceHeight   {},      resolution.width {}, resolution.height  {}   mirrored.length  {}    targetSize   {}",
//                    sourceWidth, sourceHeight, resolution.getWidth(), resolution.getHeight(), mirroredPix.size(), targetSize);
//            throw new RuntimeException("cropping crashed");
        }

//        final var croppedPpm = createPPMFile(croppedPix, resolution.getWidth(), resolution.getHeight());
//        final var fn1 = String.format("%s_orig_db_cropped_%s.ppm", filename, resolution.getName());
//        final BufferedWriter writer1 = new BufferedWriter(new FileWriter(fn1));
//        writer1.write(croppedPpm.toString());
//        writer1.close();

        // invert image pixels
        final List<Pixel> invertedPix = new ArrayList<>();

        try {
            for (int y = 0; y < resolution.getHeight(); y++) {
                for (int x = 0; x < resolution.getWidth(); x++) {
                    final var idx = y * resolution.getWidth() + x;
                    final var p = croppedPix.get(idx);
                    final var invertedPixel = new Pixel();
                    invertedPixel.setR(255 - p.getR());
                    invertedPixel.setG(255 - p.getG());
                    invertedPixel.setB(255 - p.getB());
                    invertedPix.add(invertedPixel);
                }
            }
        } catch (final Exception e) {
            LOG.error("error inverting the image   {}", e.getMessage());
        }

        try {
            final StringBuilder ppm = createPPMFile(invertedPix, resolution.getWidth(), resolution.getHeight());
            return ppm.toString();
        } catch (final Exception e) {
            LOG.error("error creating the PPM   {}", e.getMessage());
        }
        return null;
    }

    private void base64Stuff(ImageModel i, Resolution resolution, List<Image> res) throws IOException {
//        final byte[] data = Base64.getDecoder().decode(i.getImage());
//        final var output = resize(data, resolution.getWidth(), resolution.getHeight());

//        final var os = new ByteArrayOutputStream();
//        ImageIO.write(output, "png", os);
//
//        final var bytes = os.toByteArray();
//        final var im = Base64.getEncoder().encodeToString(bytes);

        final var finalImage = new Image();
        finalImage.setId(i.getId());
        finalImage.setFilename(i.getFilename());
        // finalImage.setImage(im);
        finalImage.setResolution(resolution.getName());

        res.add(finalImage);
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
