export interface ImageModel {
    id: number;
    filename: string;
    image_as_rgb_png: string;
    image_as_json_pixels_array: string;
    width: number;
    height: number;
}

export interface ArticleModel {
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


export interface Image {
    id: number;
    filename: string,
    image: string;
    resolution: string;
    width: number;
    height: number;
}

export interface Pixel {
    r: number;
    g: number,
    b: number;
}

export interface Resolution {
    id: number;
    name: string,
    width: number;
    height: number;
    original: boolean;
}


export interface Article {
    code: string;
    title: string,
    description: string;
    images: Array<Image>;
}


export interface ArticleAndImage {
    code: string;
    title: string,
    description: string;
    id: number;
    filename: string;
    image_as_json_pixels_array: string;
    width: number;
    height: number;
}
