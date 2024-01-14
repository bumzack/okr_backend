export interface ImagenModel {
    id: number;
    filename: string;
    image_as_rgb_png: string;
    image_as_json_pixels_array: string;
    width: number;
    height: number;
}

export interface ArticlenModel {
    id: number;
    code: string;
    title: string;
    description: string;
}

export interface Art2ImgModel {
    id: number;
    article_id: number;
    image_id: number;
}


export interface ResolutionModel {
    id: number;
    resolution: string;
}


