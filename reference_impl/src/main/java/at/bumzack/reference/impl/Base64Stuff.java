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
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

import static java.awt.image.BufferedImage.TYPE_3BYTE_BGR;


@Component
public class Base64Stuff {




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
