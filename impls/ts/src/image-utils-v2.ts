import {getArticles} from "./articleservice_v1";
import {getArt2Img} from "./art2imgservice_v1";
import {Article, Image, ImageModel, Pixel, Resolution} from "./models";
import {getImages} from "./imageservice_v1";


  const manipulateImage = (pixels: Array<Pixel>, width: number, height: number, targetWidth: number, targetHeight: number): string => {
    let ppm = ""
    let line = ""
    for (let yTarget = 0; yTarget < targetHeight; yTarget++) {
        for (let xTarget = 0; xTarget < targetWidth; xTarget++) {
            const xx = width - xTarget - 1
            const yy = height - yTarget - 1
            const idx = yy * width + xx
            const pixel = pixels[idx]
            const invPixel: Pixel = {
                b: 255 - pixel.b,
                g: 255 - pixel.g,
                r: 255 - pixel.r,
            }
            const pixelsAsString = `${invPixel.r} ${invPixel.g} ${invPixel.b} `
            if (line.length + pixelsAsString.length > 70) {
                ppm = ppm + line + "\n"
                line = pixelsAsString
            } else {
                line = line + pixelsAsString
            }
        }
    }
    ppm = ppm + line
    return ppm
}


  const makePpm = (pixels: String, width: number, height: number): string => {
    return `P3\n${width} ${height}\n255\n${pixels}`
}

  const processImage = (image: ImageModel, resolution: Resolution): Image => {
    const pixels = JSON.parse(image.image_as_json_pixels_array)

    const img = manipulateImage(pixels, image.width, image.height, resolution.width, resolution.height)
    const ppm = makePpm(img, resolution.width, resolution.height)

    return {
        filename: image.filename,
        height: resolution.height,
        width: resolution.width,
        id: image.id,
        image: ppm,
        resolution: resolution.name,
    }
}

  const resizeImage = (image: ImageModel, resolutions: Array<Resolution>): Array<Image> => {
    return resolutions.map(resolution => {
        if (resolution.original) {
            resolution.width = image.width
            resolution.height = image.height
        }
        return processImage(image, resolution)
    })
}

  const resizeImages = (images: Array<ImageModel>, resolutions: Array<Resolution>): Array<Image> => {
    return images.flatMap(image => {
        return resizeImage(image, resolutions)
    })
}

export const processAllImagesV2 = async (offset: number, pagesize: number, resolutions: Resolution[]): Promise<Array<Article>> => {
    const articles = await getArticles(offset, pagesize)

    return await Promise.all(articles.map(async (article) => {
        const article_id = article.id
        const art2imgs = await getArt2Img(article_id)

        const imgIds = art2imgs.map(art2img => art2img.image_id);
        const images: ImageModel[] = await getImages(imgIds)

        const resizedImages = resizeImages(images, resolutions)
        const art: Article = {
            code: article.code,
            title: article.title,
            description: article.description,
            images: resizedImages,
        }
        return art;
    }))
}
