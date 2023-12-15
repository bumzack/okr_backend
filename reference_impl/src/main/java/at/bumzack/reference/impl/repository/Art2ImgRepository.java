package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.Art2Img;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface Art2ImgRepository extends JpaRepository<Art2Img, Long> {
    List<Art2Img> findByArticleId(long  articleId);
}
