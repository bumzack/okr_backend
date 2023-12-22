package at.bumzack.reference.impl.dto;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "resolutions")
public class ResolutionModel {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private long id;

    private String resolution;


    public ResolutionModel() {
    }

    public long getId() {
        return id;
    }

    public void setId(final long id) {
        this.id = id;
    }

    public String getResolution() {
        return resolution;
    }

    public void setResolution(final String resolution) {
        this.resolution = resolution;
    }
}
