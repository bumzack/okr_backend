use common::models::ResolutionModel;
use commonbefe::models::Resolution;

pub fn to_resolution(r: &ResolutionModel) -> Resolution {
    if r.resolution.eq("original") {
        Resolution {
            name: r.resolution.clone(),
            width: -1,
            height: -1,
            original: true,
        }
    } else {
        let width_height: Vec<&str> = r.resolution.split('x').collect();
        let w = width_height[0]
            .parse::<i32>()
            .expect("width should be a number ");
        let h = width_height[1]
            .parse::<i32>()
            .expect("height should be a number ");

        Resolution {
            name: r.resolution.clone(),
            width: w,
            height: h,
            original: false,
        }
    }
}

pub fn get_sorted_resolutions(resoultions: Vec<ResolutionModel>) -> Vec<Resolution> {
    let mut originals: Vec<Resolution> = resoultions
        .iter()
        .filter(|r| r.resolution.eq("original"))
        .map(|r| to_resolution(r))
        .collect();

    let mut others: Vec<Resolution> = resoultions
        .iter()
        .filter(|r| !r.resolution.eq("original"))
        .map(|r| to_resolution(r))
        .collect();

    others.sort_by(|a, b| b.width.cmp(&a.width));

    originals.append(&mut others);

    originals
}
