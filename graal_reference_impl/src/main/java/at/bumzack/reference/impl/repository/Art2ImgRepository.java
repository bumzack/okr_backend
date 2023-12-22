package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.Art2ImgModel;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface Art2ImgRepository extends JpaRepository<Art2ImgModel, Long> {
    List<Art2ImgModel> findByArticleId(long articleId);
}
