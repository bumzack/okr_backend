package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.ImageModel;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface ImageRepository extends JpaRepository<ImageModel, Long> {
    List<ImageModel> findByIdIn(List<Long> imgIds);
}
