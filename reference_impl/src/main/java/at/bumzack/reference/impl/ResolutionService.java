package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Resolution;
import at.bumzack.reference.impl.dto.ResolutionModel;
import at.bumzack.reference.impl.repository.ResolutionRepository;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.stereotype.Component;
import org.springframework.util.CollectionUtils;
import org.springframework.util.StringUtils;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;


@Component
public class ResolutionService {

    public static final String ORIGINAL = "original";
    private static final Logger LOG = LogManager.getLogger(ResolutionService.class);
    private final ResolutionRepository resolutionRepository;

    public ResolutionService(final ResolutionRepository resolutionRepository) {

        this.resolutionRepository = resolutionRepository;
    }

    public List<Resolution> findAll() {
        final var resolutions = resolutionRepository.findAll();
        final String db = resolutions.stream()
                .map(r -> String.valueOf(r.getResolution()))
                .collect(Collectors.joining(" // "));
        //  LOG.info("all resolutions in DB     {}", db);

        final var original = resolutions.stream()
                .filter(r -> ORIGINAL.equals(r.getResolution()))
                .map(r -> toResolution(r, true))
                .toList();

        final String collect = original.stream()
                .map(Resolution::getName)
                .collect(Collectors.joining(" // "));
        // LOG.info("originals     {}", collect);

        if (CollectionUtils.isEmpty(original)) {
            LOG.info("error   originals is empty ");
        }

        final var others = resolutions.stream()
                .filter(r -> !ORIGINAL.equals(r.getResolution()))
                .map(r -> toResolution(r, false))
                .sorted(Comparator.comparingInt(Resolution::getWidth))
                .toList();

        if (CollectionUtils.isEmpty(others)) {
            LOG.info("others   originals is empty ");
        }

        final List<Resolution> response = new ArrayList<>();
        response.addAll(original);
        response.addAll(others);
        return response;
    }

    private Resolution toResolution(final ResolutionModel r, final boolean original) {
        final var res = new Resolution();
        if (original) {
            res.setId(r.getId());
            res.setName(ORIGINAL);
            res.setWidth(-1);
            res.setHeight(-1);
            res.setOriginal(true);
        } else {
            final var widthHeight = StringUtils.split(r.getResolution(), "x");
            assert widthHeight != null;
            final var w = widthHeight[0];
            final var h = widthHeight[1];

            final int scaledWidth = Integer.parseInt(w);
            final int scaledHeight = Integer.parseInt(h);

            res.setId(r.getId());
            res.setName(r.getResolution());
            res.setWidth(scaledWidth);
            res.setHeight(scaledHeight);
            res.setOriginal(false);
        }
        return res;
    }
}
