import type TextureAtlasTypes from "@/data/types/texture_atlas.d.ts";
import type SophieTypes from "@/data/types/sophie.d.ts";
import { createContext } from "react";

export const SophieContext = createContext(null as unknown as SophieData);

export type SophieData = {
    items_texture_atlas: TextureAtlasTypes.UniformTextureAtlasInfo,
    items: SophieTypes.Item[],
    present_info: SophieTypes.PresentInfo,
    rumors: SophieTypes.Rumor[],
    dolls: SophieTypes.Doll[],
};

export async function getSophieData(): Promise<SophieData> {
    const url_base = `${import.meta.env.VITE_DATA_URL}/sophie`;

    const [
        items_texture_atlas,
        items,
        present_info,
        rumors,
        dolls,
    ] = await Promise.all([
        fetch(`${url_base}/texture-atlasses/items.json`).then(res => res.json()),
        fetch(`${url_base}/items.json`).then(res => res.json()),
        fetch(`${url_base}/presents.json`).then(res => res.json()),
        fetch(`${url_base}/rumors.json`).then(res => res.json()),
        fetch(`${url_base}/dolls.json`).then(res => res.json()),
    ]);

    return {
        items_texture_atlas,
        items,
        present_info,
        rumors,
        dolls,
    };
}
