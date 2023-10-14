// AUTO-GENERATED by typescript-type-def

export default types;
export namespace types {
    export type Usize = number;
    export type I32 = number;
    export type Item = {
        "name": string;
        "tag": string;
        "image_no": types.Usize;
        "cost": types.I32;
        "use_type": string;
        "base": string;
        "level": types.Usize;
        "shape_type": string;
        "base_size": types.Usize;
        "quality_name": string;
        "size_name": string;
        "color": string;
        "category": (string)[];
        "reasonable": (types.Usize)[];
        "strengthening": (types.Usize | null);
        "hp": (types.Usize | null);
        "mp": (types.Usize | null);
        "lp": (types.Usize | null);
        "atk": (types.Usize | null);
        "def": (types.Usize | null);
        "spd": (types.Usize | null);
        "damage_min": (types.Usize | null);
        "damage_max": (types.Usize | null);
        "doll_tendency_cute": (types.I32 | null);
        "doll_tendency_wise": (types.I32 | null);
        "doll_tendency_brave": (types.I32 | null);
        "doll_tendency_fool": (types.I32 | null);

        /**
         * Player character, relates to which player cannot use this item.
         */
        "player_characters": (types.Usize)[];
    };
    export type F32 = number;
    export type PresentBasePoints = {
        "attack": types.F32;
        "heal": types.F32;
        "support": types.F32;
        "field": types.F32;
        "mix": types.F32;
        "machine": types.F32;
        "weapon": types.F32;
        "armor": types.F32;
        "accessory": types.F32;
        "material": types.F32;
    };

    /**
     * Present info for a specific friend
     */
    export type FriendPresentInfo = {

        /**
         * Gift items and their points
         */
        "item_points": Record<string, types.F32>;

        /**
         * Base points for each item type
         */
        "base_points": types.PresentBasePoints;

        /**
         * The default friendship points for this friend
         */
        "default_points": types.Usize;

        /**
         * The default friendship point limit for this friend
         */
        "default_limit": types.Usize;

        /**
         * Unlockable friendship point limits with their required events
         */
        "unlockable_limits": ([types.Usize, string])[];
    };
    export type PresentInfo = {

        /**
         * Present info for each friend
         */
        "friend_present_info": Record<string, types.FriendPresentInfo>;
    };
    export type Rumor = {

        /**
         * The name of the rumor as shown in the in-game list.
         */
        "name": string;
        "type": string;
        "group": types.Usize;
        "ev_tag": (string | null);
        "fieldmap_tag": (string | null);
        "monster_tag": (string | null);
        "item_tag": (string | null);
        "image_no": types.Usize;
        "icon_image_no": types.Usize;

        /**
         * The cost of the rumor.
         */
        "cost": types.Usize;
        "count": (types.Usize | null);
        "deadline": types.Usize;
        "interval": types.Usize;

        /**
         * Whether this rumor can be bought again after it has been completed.
         */
        "redo": boolean;
        "ev_begin": (string | null);
        "ev_end": (string | null);
        "cond_quest_group": (string | null);
        "priority": types.Usize;
        "probability": types.Usize;
        "register": boolean;

        /**
         * The title at the top of the rumor preview.
         */
        "category": string;

        /**
         * The text that is shown in the rumor preview.
         */
        "introduction": string;

        /**
         * The text spoken by the rumor seller.
         */
        "text": string;
    };
    export type SophieData = {
        "item_data": (types.Item)[];
        "present_info": types.PresentInfo;
        "rumors": (types.Rumor)[];
    };
}
