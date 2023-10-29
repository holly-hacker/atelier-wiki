import type TextureAtlasTypes from "@/data/types/texture_atlas.d.ts";
import type Ryza3Types from "@/data/types/ryza3.d.ts";
import type MapDataTypes from "@/data/types/map_data.d.ts";
import { createContext } from "react";

export const Ryza3Context = createContext(null as unknown as Ryza3Data);

export type Ryza3Data = {
    items_texture_atlas: TextureAtlasTypes.UniformTextureAtlasInfo,
    enemies_texture_atlas: TextureAtlasTypes.UniformTextureAtlasInfo,

    map_data: MapDataTypes.MapInfoList,
    field_map: Ryza3Types.FieldMapData,
    field_data: Ryza3Types.FieldData,

    items: Ryza3Types.Item[],
    item_categories: Ryza3Types.ItemCategoryData,
    item_effects: Ryza3Types.ItemEffectData,

    recipes: Ryza3Types.RecipeData,

    enemies: Ryza3Types.Enemy[],

    puni_feeding: Ryza3Types.PuniFeedingData,
    quests: Ryza3Types.QuestData,
};

export async function getRyza3Data(): Promise<Ryza3Data> {
    const url_base = `${import.meta.env.VITE_DATA_URL}/ryza3`;

    const [
        items_texture_atlas,
        enemies_texture_atlas,
        map_data,
        field_map,
        field_data,
        items,
        item_categories,
        item_effects,
        recipes,
        enemies,
        puni_feeding,
        quests,
    ] = await Promise.all([
        fetch(`${url_base}/texture-atlasses/items.json`).then(res => res.json()),
        fetch(`${url_base}/texture-atlasses/enemies.json`).then(res => res.json()),
        fetch(`${url_base}/maps/map_data.json`).then(res => res.json()),
        fetch(`${url_base}/field_map.json`).then(res => res.json()),
        fetch(`${url_base}/field_data.json`).then(res => res.json()),
        fetch(`${url_base}/items.json`).then(res => res.json()),
        fetch(`${url_base}/item_categories.json`).then(res => res.json()),
        fetch(`${url_base}/item_effects.json`).then(res => res.json()),
        fetch(`${url_base}/recipes.json`).then(res => res.json()),
        fetch(`${url_base}/enemies.json`).then(res => res.json()),
        fetch(`${url_base}/puni_feeding.json`).then(res => res.json()),
        fetch(`${url_base}/quests.json`).then(res => res.json()),
    ]);

    return {
        items_texture_atlas,
        enemies_texture_atlas,
        map_data,
        field_map,
        field_data,
        items,
        item_categories,
        item_effects,
        recipes,
        enemies,
        puni_feeding,
        quests,
    };
}