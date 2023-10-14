import type SophieTypes from "@/data/types/sophie.d.ts";
import { createContext } from "react";

export const SophieContext = createContext(null as unknown as SophieData);

export type SophieData = {
    items: SophieTypes.Item[],
    present_info: SophieTypes.PresentInfo,
};

export async function getSophieData(): Promise<SophieData> {
    const url_base = `${import.meta.env.VITE_DATA_URL}/sophie`;

    const [
        items,
        present_info,
    ] = await Promise.all([
        fetch(`${url_base}/items.json`).then(res => res.json()),
        fetch(`${url_base}/presents.json`).then(res => res.json()),
    ]);

    return {
        items,
        present_info,
    };
}