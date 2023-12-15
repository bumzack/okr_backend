package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.MyImage;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface ImageRepository extends JpaRepository<MyImage, Long> {
    List<MyImage> findByIdIn(List<Long> imgIds);
}
