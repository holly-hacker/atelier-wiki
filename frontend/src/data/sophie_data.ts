import type TextureAtlasTypes from "@/data/types/common/texture_atlas.d.ts";
import type { DollTypes, ItemTypes, PresentTypes, RumorTypes } from "@/data/types/sophie.d.ts";
import type SophieManualTypes from "@/data/types/sophie-manual.d.ts";
import { createContext } from "react";

export const SophieContext = createContext(null as unknown as SophieData);

export type SophieData = {
    items_texture_atlas: TextureAtlasTypes.UniformTextureAtlasInfo,
    items: ItemTypes.Item[],
    present_info: PresentTypes.PresentInfo,
    rumors: RumorTypes.Rumor[],
    dolls: DollTypes.Doll[],

    categories: SophieManualTypes.Categories,
    ingredients: SophieManualTypes.Ingredients,
    item_boards: SophieManualTypes.ItemBoardMap,
    shapes: SophieManualTypes.ShapeMap,
};

export async function getSophieData(): Promise<SophieData> {
    const url_base = `${import.meta.env.VITE_DATA_URL}/sophie`;

    const [
        items_texture_atlas,
        items,
        present_info,
        rumors,
        dolls,

        categories,
        ingredients,
        item_boards,
        shapes,
    ] = await Promise.all([
        fetch(`${url_base}/texture-atlasses/items.json`).then(res => res.json()),
        fetch(`${url_base}/items.json`).then(res => res.json()),
        fetch(`${url_base}/presents.json`).then(res => res.json()),
        fetch(`${url_base}/rumors.json`).then(res => res.json()),
        fetch(`${url_base}/dolls.json`).then(res => res.json()),

        fetch(`${url_base}/categories.json`).then(res => res.json()),
        fetch(`${url_base}/ingredients.json`).then(res => res.json()),
        fetch(`${url_base}/item_boards.json`).then(res => res.json()),
        fetch(`${url_base}/shapes.json`).then(res => res.json()),
    ]);

    return {
        items_texture_atlas,
        items,
        present_info,
        rumors,
        dolls,
        item_boards,
        shapes,
        categories,
        ingredients,
    };
}
