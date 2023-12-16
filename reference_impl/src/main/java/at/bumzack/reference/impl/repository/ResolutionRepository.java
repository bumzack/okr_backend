package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.ResolutionModel;
import org.springframework.data.jpa.repository.JpaRepository;

public interface ResolutionRepository extends JpaRepository<ResolutionModel, Long> {
}
