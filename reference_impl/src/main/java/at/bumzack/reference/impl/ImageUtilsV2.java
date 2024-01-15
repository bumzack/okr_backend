package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Pixel;
import at.bumzack.reference.impl.dto.Resolution;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.stereotype.Component;

import java.util.List;


@Component
public class ImageUtilsV2 {
    private static final Logger LOG = LogManager.getLogger(ImageUtilsV2.class);

    public ImageUtilsV2() {
    }

    public static String convertImageV2(final List<Pixel> pixels,
                                        final int width,
                                        final int height,
                                        final Resolution resolution) {
        try {
            if (resolution.isOriginal()) {
                resolution.setWidth(width);
                resolution.setHeight(height);
            }
            return manipulateImage(pixels, width, resolution);
        } catch (final Exception e) {
            LOG.error("error while resizing the image {}", e.getMessage());
            LOG.error(e.getCause());
        }
        return null;
    }

    private static String manipulateImage(final List<Pixel> source,
                                          final Integer sourceWidth,
                                          final Resolution resolution) {
        final StringBuilder ppm = new StringBuilder();
        try {
            StringBuilder line = new StringBuilder();

            for (int y = 0; y < resolution.getHeight(); y++) {
                for (int x = 0; x < resolution.getWidth(); x++) {
                    final var idx = y * sourceWidth + x;
                    final var p = source.get(idx);
                    final var invertedPixel = new Pixel();
                    invertedPixel.setR(255 - p.getR());
                    invertedPixel.setG(255 - p.getG());
                    invertedPixel.setB(255 - p.getB());

                    final var pixelsAsString = String.format("%d %d %d ",
                            invertedPixel.getR(), invertedPixel.getG(), invertedPixel.getB());
                    if (line.length() + pixelsAsString.length() > 70) {
                        ppm.append(line);
                        ppm.append("\n");
                        line = new StringBuilder();
                        line.append(pixelsAsString);
                    } else {
                        line.append(pixelsAsString);
                    }
                }
            }
            ppm.append(line);

        } catch (final Exception e) {
            LOG.error("error inverting the image   {}", e.getMessage());
            throw new RuntimeException("inverting crashed");
        }
        return ppm.toString();
    }
}
