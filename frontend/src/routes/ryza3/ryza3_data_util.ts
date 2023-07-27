import types from "@/atelier-data-types";

export function item_display_name(item: types.Item): String {
    // NOTE: sort is not always correct, there are some items that share a `sort` value
    if (item.name) {
        return item.name;
    }
    return item.tag ?? `Unnamed item (#${item.sort})`;
}

export function enemy_display_name(enemy: types.Enemy): String {
    return enemy.name;
}
