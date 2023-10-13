import type SophieTypes from "@/data/types/sophie.d.ts";
import { createContext } from "react";

export const SophieContext = createContext(null as unknown as SophieData);

export type SophieData = {
    items: SophieTypes.Item[],
};

export async function getSophieData(): Promise<SophieData> {
    const url_base = `${import.meta.env.VITE_DATA_URL}/sophie`;

    const [
        items,
    ] = await Promise.all([
        fetch(`${url_base}/items.json`).then(res => res.json()),
    ]);

    return {
        items,
    };
}