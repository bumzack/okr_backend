package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Pixel;
import at.bumzack.reference.impl.dto.Resolution;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.stereotype.Component;

import java.util.ArrayList;
import java.util.List;


@Component
public class ImageUtilsV1 {
    private static final Logger LOG = LogManager.getLogger(ImageUtilsV1.class);

    public ImageUtilsV1() {
    }

    public static StringBuilder createPPMFile(final List<Pixel> pixels, final int width, final int height) {
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

    public static List<Pixel> convertImage(final List<Pixel> pixels,
                                           final int width,
                                           final int height,
                                           final Resolution resolution) {
        try {
            if (resolution.isOriginal()) {
                resolution.setWidth(width);
                resolution.setHeight(height);
            }
            return manipulateImage(pixels, width, height, resolution);
        } catch (final Exception e) {
            LOG.error("error while resizing the image {}", e.getMessage());
            LOG.error(e.getCause());
        }
        return null;
    }

    private static List<Pixel> manipulateImage(final List<Pixel> source,
                                               final Integer sourceWidth,
                                               final Integer sourceHeight,
                                               final Resolution resolution) {

        final var mirrored = mirrorImage(source, sourceWidth, sourceHeight);
        final var croppedPix = cropImage(sourceWidth, resolution, mirrored);

        return invertImage(resolution, croppedPix);
    }

    private static List<Pixel> invertImage(final Resolution resolution, final List<Pixel> croppedPix) {
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
            throw new RuntimeException("inverting crashed");
        }
        return invertedPix;
    }

    private static List<Pixel> cropImage(final Integer sourceWidth, final Resolution resolution, final List<Pixel> mirrored) {
        final List<Pixel> croppedPix = new ArrayList<>();

        try {
            for (int yTarget = 0; yTarget < resolution.getHeight(); yTarget++) {
                for (int xTarget = 0; xTarget < resolution.getWidth(); xTarget++) {
                    final var idxSource = yTarget * sourceWidth + xTarget;
                    final var p = mirrored.get(idxSource);
                    croppedPix.add(p);
                }
            }
        } catch (final Exception e) {
            LOG.error("error cropping the image   {}", e.getMessage());
            throw new RuntimeException("cropping crashed");
        }
        return croppedPix;
    }

    private static List<Pixel> mirrorImage(final List<Pixel> source, final Integer sourceWidth, final Integer sourceHeight) {
        final List<Pixel> mirroredPix = new ArrayList<>();

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
            throw new RuntimeException("mirroring crashed");
        }
        return mirroredPix;
    }


}
