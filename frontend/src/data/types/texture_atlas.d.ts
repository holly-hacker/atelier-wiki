// AUTO-GENERATED by typescript-type-def

export default types;
export namespace types{
export type U32=number;

/**
 * Information about a texture atlas.
 */
export type UniformTextureAtlasInfo={
/**
 * The number of columns in the texture atlas.
 */
"columns":types.U32;
/**
 * The dimensions of each image.
 */
"image_dimensions":[types.U32,types.U32];
/**
 * An ordered list of the images that are stored in this texture atlas.
 */
"stored_images":(string)[];};
}